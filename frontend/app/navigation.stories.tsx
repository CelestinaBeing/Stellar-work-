import type { Meta, StoryObj } from "@storybook/nextjs-vite";
import { Navigation } from "./navigation";

const meta = {
  title: "Layout/Navigation",
  component: Navigation,
  parameters: {
    layout: "fullscreen",
    nextjs: {
      appDirectory: true,
      navigation: {
        pathname: "/",
      },
    },
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Navigation>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  parameters: {
    viewport: {
      defaultViewport: "desktop",
    },
  },
  render: () => (
    <div className="min-h-[220px] bg-slate-50">
      <Navigation />
    </div>
  ),
};

export const Mobile: Story = {
  parameters: {
    viewport: {
      defaultViewport: "mobile",
    },
  },
  render: () => (
    <div className="min-h-[420px] bg-slate-50">
      <Navigation />
    </div>
  ),
};
