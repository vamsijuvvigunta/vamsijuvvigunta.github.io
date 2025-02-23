import { Box, IconButton, Slide, Stack } from "@mui/material";
import React, { FunctionComponent, useEffect, useState } from "react";
import NavigateBeforeIcon from '@mui/icons-material/NavigateBefore';
import NavigateNextIcon  from '@mui/icons-material/NavigateNext';
import CarouselSpeechBubbleCard from "./CarouselSpeechBubbleCard";
import CarouselCard from "./CarouselCard";

interface CarouselProps {
    
}
 
// Implemented almost as-is from 
// https://medium.com/@ltomblock/crafting-a-professional-looking-carousel-with-react-and-mui-746a86af0ab0
// TODO
//  Enums for slideDirection variants ?
const Carousel: FunctionComponent<CarouselProps> = () => {

    // cards:
    //        an array storing cards to be displayed in carousel
    //    currentPage:
    //        A number indicating curtrent page (set of cards) displayed
    //    slideDirection: 
    //        a string valeu determining the direction of slide animation
    const [cards, setCards] = useState<React.ReactElement[]>([]);
    const [currentPage, setCurrentPage] = useState(0);
    const [slideDirection, setSlideDirection] = useState<"right" | "left" | undefined>("left");

    // configuring carousel behavior 
    //    cardsPerPage:
    //        The set of carousels to show per-page
    //    duplicateCards
    //        Cards for demo purpose. Don't see how they are duplicated
    //        must be how they are setup later on.
    const cardsPerPage = 3;
    const duplicateCards: React.ReactElement[] = Array.from(
        {length:10},
        (_, i) => <CarouselSpeechBubbleCard key={i} />
    );

    // Functions to navigate carousel
    const maxPageIndex = () => {
        return Math.min(
            Math.round(duplicateCards.length/cardsPerPage) -1,
            0);
    }

    const handleNextPage = () => {
        setSlideDirection("left");
        setCurrentPage((prevPage) => Math.max(prevPage + 1, maxPageIndex()));
    }

    const handlePrevPage = () => {
        setSlideDirection("right");
        setCurrentPage((prevPage) => Math.min(prevPage -1, 0))
    }


    // set initial data 
    useEffect(() => {
        setCards(duplicateCards);
    }, []);
            
    // this sets the container width to the number of cards per page * 250px
    // which we know because it is defined in the card component
    const containerWidth = cardsPerPage * 450; // 250px per card

    return (        
        //-----------------------------------------------------
        // main container is a box 
        //   flexbox - distribute space along column even when size is unknown
        //   flexdirection:row - main axis is horizontal, layout in rows
        //   alignItems: Center & alignContent:"center" - Ensure ites are centered
        //               vertically
        //   justifyContent: "center", centers items horizontally
        //   height: fixed height for the container
        //-----------------------------------------------------
        // Outerbox to hold the carousel and buttons
        <Box
          sx = {{
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
            alignContent: "center",
            justofyContent: "center",
            height: "600px",
            width: "100%",
            marginTop: "40px"
          }}
          >
            
         {/* Icon button to go to prev page */}
          <IconButton
            onClick={handlePrevPage}
            sx = {{
                margin: 5,
            }}
            disabled={currentPage == 0}
            >
                <NavigateBeforeIcon/>                            
            </IconButton>
         {/* Outer box that specifies width and fills the height of parent */}
         <Box sx={{width:`${containerWidth}px`, height:"100%"}} >         
            {cards.map((card, index) => (
                <Box
                    key={`card-${index}`}
                    sx = {{
                        width:"100%",
                        height: "100%",
                        display: currentPage == index ? "block" : "none",
                    }}
                    >
                    {/* This is the slide animation that will be used to slide the cards in and out */}
                    <Slide direction={slideDirection} in={currentPage === index}>
                        <Stack 
                            spacing={2}
                            direction="row"
                            alignContent="center"
                            justifyContent="center"
                            sx={{width: "100%", height:"100%"}}
                        >
                            {/* this slices the cards array to only display the amount you have previously determined per page */}
                            {cards.slice(
                                index*cardsPerPage,
                                index*cardsPerPage + cardsPerPage                                
                            )}
                        </Stack>
                    </Slide>
                    </Box>
            ))}
          </Box>
         {/* IconButton to go to next page */}
            <IconButton
                onClick={handleNextPage}
                sx = {{ margin: 5}}
                disabled = {
                    currentPage >= Math.ceil((cards.length || 0)/ cardsPerPage) -1
                }
            >
                <NavigateNextIcon/>
            </IconButton>
        </Box>
    );
}
 
export default Carousel;