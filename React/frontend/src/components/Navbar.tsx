import { DirectionsRun } from '@mui/icons-material';
import { AppBar, Button, IconButton, Stack, Toolbar, Typography } from '@mui/material';
import { FunctionComponent} from 'react';
import NavbarSearch from './NavSearchBar';
import { AuthIsSignedIn } from '@/contexts/AuthContext';
import AccountMenu from './auth/AccountMenu';
import { useNavigate } from 'react-router-dom';

interface NavBarProps {
    
}
 
const NavBar: FunctionComponent<NavBarProps> = () => {    
    const navigate    = useNavigate();

    function onAfterLogout() {        
       navigate("/login");
    }

    return (  
        <AppBar position='static'>
            <Toolbar>                
                <IconButton size='large' edge='start' color='inherit' aria-label='logo'>
                    <DirectionsRun/>
                </IconButton>

                <Typography variant='h6' component='div' sx={{flexGrow:1}}>
                    Hillops
                </Typography>

                <NavbarSearch/>
                
                <Stack direction='row' spacing={2}>                    
                    <Button color='inherit'>About</Button>
                    <AuthIsSignedIn>
                        <AccountMenu navigateAfterLogout={onAfterLogout}/>
                    </AuthIsSignedIn>
                </Stack>
            </Toolbar>
        </AppBar>
    );
}
 
export default NavBar;