# Licensing and branding

Checked 2026-11-02 against this checkout's [license](../LICENSE.txt), the
[Macro FAQ](https://docs.macro.com/faq), and the
[GNU license FAQ](https://www.gnu.org/licenses/gpl-faq.en.html).

The inspected baseline is under GNU AGPLv3. It permits modification and
self-hosting. A separately named workspace can be limited to the team's users;
source-code visibility and application access are separate decisions.

## Conditions to preserve

- Retain the AGPL license, applicable copyright notices and third-party notices.
- Identify the adaptation and record changes rather than implying upstream
  authorship or endorsement of this deployment.
- When providing a modified version over a network, prominently offer its
  corresponding source to those users under AGPLv3, at no charge. Keep the source
  available for the exact deployed version, including required build material.
- A public GitHub repository is one way to supply source; it does not require
  publishing team documents, chats, credentials or other application data.
- A new name, logo and theme do not change upstream copyright or the license.
  Use original branding and review the chosen name before public release.
- Review licenses and configurations of integrated services separately. The
  upstream hosted product's service arrangements do not automatically cover an
  independent deployment. If a planned distribution or deployment cannot comply
  with the AGPL's applicable source-access and other conditions, obtain an
  alternative license from the relevant rights holders. Private unreleased
  modifications are not automatically prohibited.

## Implementation direction

The chosen product name is The KEY Team. Compare the Dark glass and Minimal black
[concepts](design/README.md) before implementing a visual direction. Customize the
existing theme and branding entry points; avoid broad internal identifier renames
that create unnecessary divergence. Keep license/attribution and a source-access
entry in the product's appropriate legal/about surface. No rebranding changes have
been implemented yet.
