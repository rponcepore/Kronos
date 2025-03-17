const Navbar = () => (
    <nav className="bg-black text-white p-4 flex justify-between">
      <div className="text-xl font-bold">KRONOS</div>
      <div className="flex space-x-4">
        {["Overview", "Dashboard", "Plans", "Task Management", "Calendar", "Analytics", "Search", "Settings"].map(
          (item) => (
            <a key={item} href="#" className="hover:underline">
              {item}
            </a>
          )
        )}
      </div>
    </nav>
  );
  
  export default Navbar;

  