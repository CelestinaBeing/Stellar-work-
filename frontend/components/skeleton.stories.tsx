import type { Meta, StoryObj } from "@storybook/nextjs-vite";
import JobCardSkeleton from "./JobCardSkeleton";

const meta = {
  title: "Components/Skeleton",
  component: JobCardSkeleton,
  tags: ["autodocs"],
  argTypes: {
    compact: {
      control: "boolean",
      description: "Use the denser card layout shown in compact job lists.",
    },
  },
} satisfies Meta<typeof JobCardSkeleton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    compact: false,
  },
};

export const Compact: Story = {
  args: {
    compact: true,
  },
};

export const LoadingGrid: Story = {
  render: () => (
    <div className="grid w-[900px] gap-4 md:grid-cols-3">
      <JobCardSkeleton />
      <JobCardSkeleton />
      <JobCardSkeleton compact />
    </div>
  ),
};
