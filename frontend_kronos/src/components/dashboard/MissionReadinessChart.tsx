import React from "react";
import { PieChart, Pie, Cell, ResponsiveContainer } from "recharts";

const data = [
  { name: "Ready", value: 60, color: "#4CAF50" },
  { name: "Caution", value: 14, color: "#FFC107" },
  { name: "Critical", value: 26, color: "#F44336" }
];

const MissionReadinessChart = () => (
  <ResponsiveContainer width="100%" height={200}>
    <PieChart>
      <Pie data={data} dataKey="value" nameKey="name" outerRadius={50}>
        {data.map((entry, index) => (
          <Cell key={`cell-${index}`} fill={entry.color} />
        ))}
      </Pie>
    </PieChart>
  </ResponsiveContainer>
);

export default MissionReadinessChart;
