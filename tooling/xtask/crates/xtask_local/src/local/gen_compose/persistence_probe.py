"""Exercise local AWS state across restart/recreation; synthetic data only."""

import base64
import hashlib
import json
import sys
import urllib.request

import boto3


def main():
    session = boto3.Session(
        aws_access_key_id="test", aws_secret_access_key="test", region_name="us-east-1"
    )
    clients = {
        name: session.client(name, endpoint_url="http://localhost:4566")
        for name in ("s3", "sqs", "dynamodb", "kms")
    }
    s3, sqs, ddb, kms = (clients[name] for name in ("s3", "sqs", "dynamodb", "kms"))
    bucket = "macro-persistence-regression"
    body = b"Storage restart regression\n" + bytes(range(256)) * 4
    digest = hashlib.sha256(body).hexdigest()

    if sys.argv[1] == "seed":
        s3.create_bucket(Bucket=bucket)
        queue = sqs.create_queue(QueueName=bucket)["QueueUrl"]
        sqs.send_message(QueueUrl=queue, MessageBody="retained queue message")
        queue_arn = sqs.get_queue_attributes(
            QueueUrl=queue, AttributeNames=["QueueArn"]
        )["Attributes"]["QueueArn"]
        s3.put_bucket_notification_configuration(
            Bucket=bucket,
            NotificationConfiguration={"QueueConfigurations": [{
                "QueueArn": queue_arn, "Events": ["s3:ObjectCreated:*"]
            }]},
        )
        s3.put_object(Bucket=bucket, Key="fixture.bin", Body=body,
                      ContentType="application/octet-stream", Metadata={"sha256": digest})
        ddb.create_table(
            TableName=bucket,
            KeySchema=[{"AttributeName": "file_id", "KeyType": "HASH"}],
            AttributeDefinitions=[{"AttributeName": "file_id", "AttributeType": "S"}],
            BillingMode="PAY_PER_REQUEST",
        )
        ddb.get_waiter("table_exists").wait(TableName=bucket)
        ddb.put_item(TableName=bucket, Item={"file_id": {"S": "fixture"}, "sha256": {"S": digest}})
        key = kms.create_key()["KeyMetadata"]["KeyId"]
        ciphertext = kms.encrypt(KeyId=key, Plaintext=body)["CiphertextBlob"]
        print(json.dumps({"ciphertext": base64.b64encode(ciphertext).decode()}))
        return

    expected = json.loads(sys.argv[2])
    obj = s3.get_object(Bucket=bucket, Key="fixture.bin")
    assert obj["Body"].read() == body
    assert obj["Metadata"]["sha256"] == digest
    assert obj["ContentType"] == "application/octet-stream"
    url = s3.generate_presigned_url(
        "get_object", Params={"Bucket": bucket, "Key": "fixture.bin"}, ExpiresIn=60
    )
    with urllib.request.urlopen(url, timeout=10) as response:
        assert response.read() == body
    assert ddb.get_item(TableName=bucket, Key={"file_id": {"S": "fixture"}})["Item"]["sha256"]["S"] == digest
    queue = sqs.get_queue_url(QueueName=bucket)["QueueUrl"]
    notifications = s3.get_bucket_notification_configuration(Bucket=bucket)
    assert notifications["QueueConfigurations"][0]["Events"] == ["s3:ObjectCreated:*"]
    messages = sqs.receive_message(QueueUrl=queue, MaxNumberOfMessages=10, VisibilityTimeout=0)
    assert any(message["Body"] == "retained queue message" for message in messages["Messages"])
    assert kms.decrypt(CiphertextBlob=base64.b64decode(expected["ciphertext"]))["Plaintext"] == body
    print("PASS: object bytes, presigned download, metadata, queue, notification configuration and KMS")


if __name__ == "__main__":
    main()
