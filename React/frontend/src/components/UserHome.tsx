import { FunctionComponent, useEffect, useState } from "react";
import Carousel from "@/components/carousel/Carousel";

// test
import { OneShotMsg } from "@/api/llm";
// ------

interface UserHomeProps {
    
}
 
const UserHome: FunctionComponent<UserHomeProps> = () => {

    const [llmResponse, setLlmResponse] = useState("")

    useEffect( () => {
        async function fetchLlmResponse() {
            let llm_response = await OneShotMsg("Why is the sky blue", "User");
            setLlmResponse(llm_response || "");
        }

        fetchLlmResponse();
    }, [])

    

    return ( 
    <>
        <h1>Welcome home user!</h1>
        <Carousel/>
        <hr/>
        <h2>{llmResponse}</h2>
    </>);
}
 
export default UserHome;