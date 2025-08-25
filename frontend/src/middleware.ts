import { defineMiddleware } from 'astro:middleware';

// API routes that need CORS handling
const API_ROUTES = ['/api'];

export const onRequest = defineMiddleware(async (context, next) => {
  const { request, url } = context;
  
  // Handle CORS for API routes
  if (API_ROUTES.some(route => url.pathname.startsWith(route))) {
    const response = await next();
    
    // Add CORS headers
    response.headers.set('Access-Control-Allow-Origin', '*');
    response.headers.set('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
    response.headers.set('Access-Control-Allow-Headers', 'Content-Type, Authorization');
    
    // Handle preflight requests
    if (request.method === 'OPTIONS') {
      return new Response(null, { status: 200, headers: response.headers });
    }
    
    return response;
  }
  
  // For now, let client-side handle authentication
  // This avoids the server-side token validation issues
  return next();
});
