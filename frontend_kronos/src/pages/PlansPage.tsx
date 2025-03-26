import React, { useEffect, useState } from "react";
import PlansList from "../components/PlansList";
import PlanDetails from "../components/PlansDetails";
import { Plan } from "../types/planTypes";
import { KronosRequest } from "../types/KronosRequest";
import { KronosResponse } from "../types/KronosResponse";
import { kronosApiCall } from "../helper_methods/ApiCall";


const PlansPage: React.FC = () => {
  const [plansData, setPlansData] = useState<Plan[]>([]);
  const [selectedPlan, setSelectedPlan] = useState<Plan | null>(null);
  const [searchTerm, setSearchTerm] = useState("");
  const [sortBy, setSortBy] = useState<"default" | "alpha" | "published">("default");
  const [filterYear, setFilterYear] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchPlans() {
      try {
        setLoading(true);
        setPlansData([]); // Clear existing plans data before fetching new data
        const req: KronosRequest = { action: "GET_PLANS", unit: "", plan_id: 0, order_id: 0, paragraph_id: 0, task_id: 0 };
        const res: KronosResponse = await kronosApiCall(req);
          setPlansData(res.plans_vec); // Update the state with the fetched plans
      } catch (error) {
        console.error("Error fetching plans:", error);
      } finally {
        setLoading(false);
      }
    }

  
    fetchPlans();
  }, []);
  

  const basePlans = plansData.filter((p) => p.type === "PLAN");

  const filteredPlans = basePlans
    .filter((p) => !filterYear || p.number.startsWith(filterYear))
    .filter((p) =>
      p.number.toLowerCase().includes(searchTerm.toLowerCase()) ||
      p.mission.toLowerCase().includes(searchTerm.toLowerCase())
    )
    .sort((a, b) => {
      if (sortBy === "alpha") return a.number.localeCompare(b.number);
      if (sortBy === "published") return a.published.localeCompare(b.published);
      return 0;
    });

  if (loading) {
    return <div className="p-6 mt-16">Loading plans...</div>;
  }

  return (
    <div className="p-6 mt-16">
      {selectedPlan ? (
        <PlanDetails
          plan={selectedPlan}
          allPlans={plansData}
          goBack={() => setSelectedPlan(null)}
        />
      ) : (
        <>
          <div className="top-controls mb-6 flex gap-4 flex-wrap">
            <button className="control-btn" onClick={() => setFilterYear(null)}>All Years</button>
            <button className="control-btn" onClick={() => setFilterYear("25")}>FY25</button>
            <button className="control-btn" onClick={() => setSortBy("alpha")}>Order A-Z</button>
            <button className="control-btn" onClick={() => setSortBy("published")}>Order by Published</button>
            <input
              type="text"
              className="control-input"
              placeholder="Search plans..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
            <button className="control-btn">New Plan ➕</button>
          </div>

          <PlansList plans={filteredPlans} selectPlan={setSelectedPlan} hideTypeBadge />
        </>
      )}
    </div>
  );
};

export default PlansPage;
