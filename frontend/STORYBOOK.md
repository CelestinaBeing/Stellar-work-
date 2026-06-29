# Storybook

Storybook documents the reusable UI components and product patterns used by
StellarWork. It gives contributors an isolated playground for props, responsive
states, accessibility checks, and visual regression snapshots.

## Commands

```bash
npm run storybook
npm run build-storybook
npm run chromatic
```

## Coverage

Stories currently cover:

- Section cards, status badges, error banners, empty states, loading states, tooltips, and skeletons.
- Confirmation dialogs and toast notifications.
- Navigation layout across desktop and mobile viewports.
- Job card, activity timeline, and announcement banner product patterns.

## Accessibility and Viewports

`@storybook/addon-a11y` is enabled globally. The preview config also defines
mobile, tablet, and desktop viewports so contributors can review responsive
states without leaving Storybook.

## Visual Regression

The Storybook workflow builds every PR. When `CHROMATIC_PROJECT_TOKEN` is
configured in repository secrets, Chromatic runs against the static build for
visual regression coverage. On pushes to `main`, the static Storybook build is
published to GitHub Pages.
