import {IUser} from "../model/iuser.interface";
//import {API_URL} from "./env";

import axios from "axios";

// ??
export async function AuthenticateRequest(): Promise<IUser|null> {
    const signedInUser = localStorage.getItem("username");
    if ( signedInUser ) {
        console.log(`AuthRequest: Have signedinuser of ${signedInUser}`);
        return {username: signedInUser, first: "John", last: "Doe"};
    } else {
        return null;
    }
}

export async function LoginRequest(username: string, pwd: string, rememberMe: boolean) : Promise<IUser|null> {
        
    try {
        const login_url = "/api/login";
        const response = await axios.post(login_url, 
            {
            "username": username,
			"pwd": pwd
          }        
        )
  
        // The api call returns the following json (See lib_web/handlers/handlers_login)
        // "result": {
		//	    "success": true
		//  }
        if (response.data.result.success) {
            console.log("Login succeeded");
            console.log(`Response headers ${response.headers}`)

            localStorage.setItem("username", username);
            return AuthenticateRequest();
        } 
        else
        {
            console.error("Login failed");            
        }
      } catch(error) {
        console.error(error)
      }
    
    // Successful login wouldve returned. If we reach here, login has failed.
    localStorage.removeItem("username");
    throw new Error("Login failed");    
}

export async function LogoutRequest() {
    
    // Mock impl.
    localStorage.removeItem("username");

    const logoff_url = "/api/logoff";
    console.log(`Hitting ${logoff_url}`)
    await axios.post(logoff_url, {"logoff": true });
}
