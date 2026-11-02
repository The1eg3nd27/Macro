# Concept verification — 2026-11-02

Scope: the static comparison, not the Macro runtime. All content is synthetic.

| Result | Check | Evidence |
| --- | --- | --- |
| Pass | Both desktop directions | Dark glass and Minimal black opened in the browser; screenshots alongside this file |
| Pass | Responsive layout | Desktop at 1100px; stacked content at 736px; mobile at 320px. At 736/320px, body scroll width equaled available width |
| Pass | Navigation | Today, Chat, Tasks, Calendar, Team Brain and Files opened |
| Pass | Task completion | Completing the visual-direction task changed open count, board placement and calendar status |
| Pass | Chat suggestion | Confirmation created one campaign-proposal task with owner/date; Tasks and Calendar reflected it |
| Pass | List/Board | The task views retained the completed and newly confirmed records |
| Pass | Task search | Searching “proposal” showed the matching task |
| Pass | Knowledge approval | Source text displayed; approval produced revision 2 and updated Today |
| Pass | Knowledge dismissal | After reset, dismissal retained revision 1 and cleared the pending update |
| Pass | AI concept | Context panel opened; model selector offered Claude/GPT/local. Response clearly labeled illustrative |
| Pass | Reset | Reload restored initial tasks and pending knowledge proposal |
| Not run | Real providers, MCP, storage and background jobs | This concept has no backend connections; use the runtime acceptance gates in the blueprint |

The first attempt to launch a separate headless browser failed in the local
environment. Verification used the available in-app browser instead; no extra
browser package was installed. No Macro containers were started or reset.

For repeatable manual steps, see [the comparison README](README.md). The existing
application's limited runtime evidence remains in [test results](../test-results.md).
