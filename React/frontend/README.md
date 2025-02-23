# Project creation and adjustment
## React + TypeScript + Vite

This template provides a minimal setup to get React working in Vite with HMR and some ESLint rules.

Currently, two official plugins are available:

- [@vitejs/plugin-react](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react/README.md) uses [Babel](https://babeljs.io/) for Fast Refresh
- [@vitejs/plugin-react-swc](https://github.com/vitejs/vite-plugin-react-swc) uses [SWC](https://swc.rs/) for Fast Refresh

## Expanding the ESLint configuration

If you are developing a production application, we recommend updating the configuration to enable type aware lint rules:

- Configure the top-level `parserOptions` property like this:

```js
export default {
  // other rules...
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: ['./tsconfig.json', './tsconfig.node.json'],
    tsconfigRootDir: __dirname,
  },
}
```

- Replace `plugin:@typescript-eslint/recommended` to `plugin:@typescript-eslint/recommended-type-checked` or `plugin:@typescript-eslint/strict-type-checked`
- Optionally add `plugin:@typescript-eslint/stylistic-type-checked`
- Install [eslint-plugin-react](https://github.com/jsx-eslint/eslint-plugin-react) and add `plugin:react/recommended` & `plugin:react/jsx-runtime` to the `extends` list


# Getting Started

 Started looking at some react/material-ui tutorials and I think, going to get the shape going with some basic toolbar (simply copy and later grok)

## Running the server

`npm run dev` and click on the URL provided.

# Overall UX concept

I struggle to describe what I am trying to achieve and bounce around the various possible use-cases. Following feedback, I need to focus on `the use-case` and get that started. 

I am having trouble with this also 🫨 so will punt for a bit. To compensate, I will try to apply `do things that do not scale` as much as possible.
 
## Use Case based

 - Multiple use cases on the landing page.
 - Each with static js/images etc so I will be developing top-down.
 - Explore the ux motivation and use-cases but pick one for back-end implementation.
 

# Development History

## Create project

Use `npm create vite@latest` to create the basic project
  - React
  - Typescript

## Include MUI

See [](../../../code_general/react/docs/MaterialUI%20-%20FAQ.md) aboout

 - Adding MaterialUI
 - Baselining CSS

## Create the Navbar

 - Pretty much copied from [MUI #21 - Navbar](https://www.youtube.com/watch?v=y9iX6sfB40k&list=PLC3y8-rFHvwh-K9mDlrrcDywl7CeVL2rO&index=21)
  - changed Pokemon icone to runner from the list at https://mui.com/material-ui/material-icons/?query=run

> Note that this looked like crap when I started out. Had to remove the main index.css and App.css from the vite template and then use CssBaseline component to clear css out.

```tsx
function App() {
  return (
    <CssBaseline>
      <MuiNavBar/>
    </CssBaseline>
  )
}
```

Good starting point now.

## Adding a search bar

 Random search got me to https://purecode.ai/blogs/mui-appbar. Added it this way, via a new component.

```tsx
 const MuiNavbarSearch: FunctionComponent<MuiNavbarSearchProps> = () => {
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
```

```diff
const MuiNavBar: FunctionComponent<MuiNavBarProps> = () => {
    return (  
        <AppBar position='static'>
            <Toolbar>                
                <IconButton size='large' edge='start' color='inherit' aria-label='logo'>
                    <DirectionsRun/>
                </IconButton>

                <Typography variant='h6' component='div' sx={{flexGrow:1}}>
                    Hillops
                </Typography>

+               <MuiNavbarSearch/>
                
                <Stack direction='row' spacing={2}>
                    <Button color='inherit'>Features</Button>
                    <Button color='inherit'>Pricing</Button>
                    <Button color='inherit'>About</Button>
                    <Button color='inherit'>Login</Button>
                </Stack>
            </Toolbar>
        </AppBar>
    );
}
```

 - Not centered
 - Search bar is too tall, but search semantics and stuff is there.

## Initial layout


### Questions

In the following

```tsx
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
          </Box>
  ))}
```

Why is the following not ok ? I would have thought you need a `{` to start the function body of the lambda ?

```tsx
{cards.map((card, index) => {
```