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
        setPlansData([]);
  
        const isDev = import.meta.env.DEV;
        const unit =  isDev ? "tstUIC" : "WJH8C0"; // use fake data only in dev
  
        const req: KronosRequest = {
          action: "get_plans",
          unit: unit,
          plan_id: 0,
          order_id: 0,
          paragraph_id: 0,
          task_id: 0
        };
  
        const res: KronosResponse = await kronosApiCall(req);

        //test block for frontend adding features
        const processedPlans = (res.plans_vec ?? []).map((p) => ({
          ...p,
          number: `FY${p.fiscal_year}-${p.serial_number.toString().padStart(3, "3")}`,
          date: `${p.fiscal_year}-10-01`,
          published: `20${p.fiscal_year.toString().slice(-2)}-10-01`,
          expires: `20${(p.fiscal_year + 1).toString().slice(-2)}-10-01`,
          type: "PLAN",
          mission: p.name,
          taskedBy: "Command HQ", // placeholder
          taskedTo: p.unit       // fallback
        }));

        //end test block
        setPlansData(processedPlans);

        
        //setPlansData(res.plans_vec ?? []);
      } catch (error) {
        console.error("Error fetching plans:", error);
      } finally {
        setLoading(false);
      }
    }
  
    fetchPlans();
  }, []);
  

  //const basePlans = plansData.filter((p) => p.type === "PLAN");
  const basePlans = plansData;
  const filteredPlans = basePlans
  .filter((p) => !filterYear || String(p.fiscal_year).endsWith(filterYear))
  .filter((p) => {
    const name = p.name?.toLowerCase() ?? "";
    const classification = p.classification?.toLowerCase() ?? "";
    return name.includes(searchTerm.toLowerCase()) || classification.includes(searchTerm.toLowerCase());
  })
  .sort((a, b) => {
    if (sortBy === "alpha") return a.name.localeCompare(b.name);
    if (sortBy === "published") return (a as any).published?.localeCompare((b as any).published ?? "") ?? 0;
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
