# Component references and visual refinement

Reviewed: 2026-11-02. This is a design study, not an installed component library.

## Direction

The initial concepts felt too generic and AI-generated. The revised comparison
keeps both directions but removes the promotional heading, coloured glow, large
rounded cards and sparkle icons. Tasks use compact, aligned rows. Calendar dates
and the agenda occupy one quiet column. Knowledge approval remains an ordinary
work item. Glass is limited to the navigation; Minimal black remains flat.

The company supplied its existing logo. The concepts now use that exact image
instead of the provisional letter badge. The product name remains The KEY Team.
A final theme has not been approved.

## Shortlist from 21st.dev

| Reference | Useful pattern | Adaptation boundary |
| --- | --- | --- |
| [shadcn Sidebar](https://21st.dev/@shadcn/components/sidebar) | Grouped navigation, compact active state and footer | Preserve Macro's shell; re-create only needed styling using its existing primitives |
| [Sidebar Light](https://21st.dev/@inference-sh/components/sidebar-light) | Simple hierarchy, labels and active route | Useful as a restrained navigation reference; not a replacement dependency |
| [shadcn Data Table](https://21st.dev/@shadcn/components/data-table) | Aligned rows, columns and filtering | Apply task/owner/due alignment to existing tasks; the mockup does not import a table library |
| [Calendar with event list](https://21st.dev/@clevision/components/calendar-1/calendar-with-event-list) | Month grid paired with an agenda | Show meetings and deadlines together while keeping their different meanings |

The pages expose descriptions, usage examples and previews; some component source
requires an account. The sidebar's embedded preview was inspected in the browser;
other patterns were assessed from their public descriptions/usage examples.
No account was created, no gated source was retrieved, and no installer, CLI, MCP
connection or package was added.

21st.dev describes these as React/TypeScript/Tailwind components. Macro's frontend
uses SolidJS. The appropriate first step is to apply the useful interaction and
layout patterns to the existing code, not paste React hooks or add a second UI
framework. No third-party implementation code was copied into these concepts.

The Sidebar, Sidebar Light and Data Table pages display MIT licensing. If code is
later adopted, review the actual source/dependency licenses and retain required
notices. Calendar source licensing was not established in this review; treat it
as a visual reference unless that separate check is completed.

## Logo handling

- [Original image](assets/the-key-original.png): the supplied 4000 × 4000 PNG,
  copied without pixel changes. It is also embedded in the fragment so the inline
  comparison needs no image-hosting service.
- Presentation uses a local SVG colour-matrix filter on the original image:
  white RGB, alpha equal to inverse luminance. This turns dark strokes white and
  makes the white background transparent without redrawing any letter outline.
  CSS clips the excess canvas whitespace; TEAM is a separate interface label.
- This is a display treatment, not a delivered vector logo or a separately
  exported transparent raster asset. A future original SVG would be preferable
  for production scaling, but is not needed to inspect the current concepts.
- A built-in image-editing attempt was rejected for visible artifacts. The
  generated result is not used or published. Its edit prompt requested only black
  strokes to white, true transparency, exact original geometry and no added text.

The logo is a supplied company brand asset, separate from Macro's upstream branding.
No trademark ownership or third-party permission is implied by this design study.

## Remaining scope

The revised calendar is a static November 2026 example. Existing local prototype
interactions remain: navigation, task confirmation/completion, list/board search,
knowledge review and an illustrative assistant panel. No model calls, calendar
sync or application service changes are part of this revision.
