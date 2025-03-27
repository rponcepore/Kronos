export const getOrderSerialDisplay = (
    fiscalYear: number,
    planSerial: number,
    orderSerial: number | null
  ) => {
    const fy = String(fiscalYear).slice(-2).padStart(2, "0");
    const ps = String(planSerial).padStart(2, "0");
    const os = String(orderSerial ?? 0).padStart(2, "0");
    return `${fy}-${ps}-${os}`;
  };
  
  export const getPlanSerialDisplay = (fiscalYear: number, planSerial: number) => {
    const fy = String(fiscalYear).slice(-2).padStart(2, "0");
    const ps = String(planSerial).padStart(2, "0");
    return `${fy}-${ps}`;
  };
  