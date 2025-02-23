import SearchIcon from '@mui/icons-material/Search'
import { InputAdornment, TextField } from '@mui/material';
import { FunctionComponent } from 'react';

interface NavbarSearchProps {
    
}
 
const NavbarSearch: FunctionComponent<NavbarSearchProps> = () => {
    return (  
        <>        
        <TextField
            variant='filled' 
            placeholder='Search..'
            InputProps={{
                startAdornment:(
                    <InputAdornment position="start">
                        <SearchIcon/>
                    </InputAdornment>
                )                                    
            }}
        />
        </>
    );
}
 
export default NavbarSearch;