import React from "react";
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer } from "recharts";

const data = [
  { name: "Sun", value: 30 },
  { name: "Mon", value: 50 },
  { name: "Tue", value: 70 },
  { name: "Wed", value: 40 },
  { name: "Thu", value: 90 },
  { name: "Fri", value: 60 },
  { name: "Sat", value: 30 }
];

const WeeklyActivityChart = () => (
  <ResponsiveContainer width="100%" height={200}>
    <LineChart data={data}>
      <XAxis dataKey="name" />
      <YAxis />
      <Tooltip />
      <Line type="monotone" dataKey="value" stroke="#82ca9d" />
    </LineChart>
  </ResponsiveContainer>
);

export default WeeklyActivityChart;
