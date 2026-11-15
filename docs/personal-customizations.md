# Personal build customizations

Keep `personal/local-customizations` separate from upstream tracking branches.
Merge reviewed upstream changes into it or replay the customization commits on
the reviewed release. Do not reset it to upstream or install an unmodified build.

## Interface without social links

This build removes Discord links from the interface, native menus, command
registries, CLI and MCP wherever present.
Project documentation and source-code links remain available.

The promotional community card is also removed from the Home page.

## Centrally managed updates

This installation is updated by its owner through the central build and deployment
workflow. The application does not start a release-check worker, display update
popups, open update links, or replace its executable. Legacy host methods remain
as no-ops. Keep this policy when merging upstream changes; enabling an in-app
updater could replace the reviewed build and discard local customizations.
