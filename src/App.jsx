import { Routes, Route, BrowserRouter, useLocation } from "react-router-dom";
import MenuPage from "./pages/MenuPage";
import TaskPage from "./pages/TaskPage";

function App() {
  const location = useLocation();

  return (
      <Routes location={location} key={location.pathname}>
        <Route index path="/" element={<MenuPage/>} />
        <Route path="/:id" element={<TaskPage/>}/>
      </Routes>
  )
}

export default App;
