export const isValidNumber = (value: unknown): value is number => {
  return (
    typeof value === "number" && !Number.isNaN(value) && Number.isFinite(value)
  );
};
