//! PlanDemoComponent.tsx
import PlanDemoCard from "./PlanDemoCard";

export default function PlanDemoComponent() {
    return (
        <div>
            <button>
                <p>Press to send a request to the api for action:get_plans</p>
            </button>
            <h2>Plan Return:</h2>
            <PlanDemoCard/> 
        </div>
    );
}

