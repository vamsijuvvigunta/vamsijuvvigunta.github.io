import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import Typography from "@mui/material/Typography";

import style from '../../css/speech_bubbles.module.css';

import thinkingImg from '@assets/thinking-person-8.197x256.png';

export default function CarouselSpeechBubbleCard() {  

  return (    

    <Card sx={{ width: "450px", height: "600px", position:"relative" }}>      
      <CardContent sx={{position:"absolute", bottom:160, left:50}}>        
        <Typography variant="body2" color="text.secondary" component="p" className={`${style.bubble} ${style.thought}`}>
          Lizards are a widespread group of 
          squamate reptiles, with over 6,000 
          species, ranging across all continents
          except Antarctica
        </Typography>                
      </CardContent>                
      <img
            style={{ height: 150, margin:10, position:"absolute", bottom:0}}
            src={thinkingImg}            
          />        
    </Card>
  );
}