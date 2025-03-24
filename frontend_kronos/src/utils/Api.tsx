
export default function ApiButton( { label, route }: { label: string; route: string } ) {
    function handleClick() {
        const fullroute: String = `localhost:8000${route}`;
        alert(`Sending request to ${fullroute}`);
    }

    return (
        <button onClick={ handleClick }>
            { label }
        </button>
    );
}