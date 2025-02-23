// import {trimCharFromFront} from '../util/string_util';

// // Only variables that match VITE_ will be exposed by Vite
// // And via this confusing import.meta.env. dictionary.
// export const API_BASE_URL = import.meta.env.VITE_BACKEND_URL;

// // Route is optional. If set, will be appended to the backend URL
// // The route will be trimmed before being appended to the base_url.
// export function API_URL(route?: string): string {
//     if (typeof route != 'undefined') {
//         const trimmed = route.trim();
//         const cleaned = trimCharFromFront(trimmed, '/');
//         return `${API_BASE_URL}/${cleaned}`;
//     }
//     else
//         return API_BASE_URL;
// }