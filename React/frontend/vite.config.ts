import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react'
import tsconfigPaths from "vite-tsconfig-paths";

// https://vitejs.dev/config/
export default defineConfig(({command, mode}) => {  

  // Loads envs from the .env.$mode file
  const env = loadEnv(mode, process.cwd());

  return {  
      plugins: [react(), 
        tsconfigPaths()
      ],
      server: {
        proxy: {
          // Target is the backend API URL
          // covers auth endpoints at /api/logon, /api/logoff
          // and the JRPC endpoint at /api/rpc.
          '/api/': {
              target: env.VITE_BACKEND_URL, 
              changeOrigin: true,          
        },
      }
    }
  }
})
