import { CssBaseline } from '@mui/material'
import NavBar from '@/components/Navbar'
import LoginForm from '@/components/auth/LoginForm'
import UserHome from '@/components/UserHome';
import AboutApp from '@/components/About';

import {
  AuthProvider,
  AuthIsNotSignedIn, 
  AuthIsSignedIn,
} from "@/contexts/AuthContext";

import {
  Navigate,
  Outlet,
  Route,
  BrowserRouter as Router,
  Routes,
} from "react-router-dom";

function App() {
  return (
    
    // CssBaseline: Reset CSS styles to a baseline. Without this there is some random styling 
    // that affects the navbar and such.
    //
    // Everything has access to authentication context because it is all wrapped in the 
    // AuthProvider component
    //
    // Page Shell/Layout is in the top / route and it wraps the output of the child 
    // Route nodes inside the layout page. See Layout() below
    <CssBaseline>
    <AuthProvider>      
        <AuthIsSignedIn>
          <Router>
            <Routes>
              <Route path="/"        element={<Layout/>}>
                  <Route path={"home"}   element={<UserHome/>} />
                  <Route path={"*"}      element={<Navigate replace to={"/home"}/>} />
              </Route>
            </Routes>
          </Router>
        </AuthIsSignedIn>

        <AuthIsNotSignedIn>
          <Router>
            <Routes>
              <Route path="/" element={<Layout/>}>
                  <Route path={"login"} element={<LoginForm postLoginRoute="/home"/>} />
                  <Route path={"about"} element={<AboutApp/>} />
                  <Route path={"*"}     element={<Navigate replace to={"/login"}/>} />
              </Route>
            </Routes>
          </Router>          
        </AuthIsNotSignedIn>

    </AuthProvider>
    </CssBaseline>
  )
}

function Layout() {
  return (
    <>
      <NavBar/>
      <Outlet/>
    </>
  );
}

export default App
