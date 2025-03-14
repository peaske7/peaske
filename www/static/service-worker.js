// Service worker for caching images
const CACHE_NAME = 'peaske-image-cache-v1';
const IMAGE_CACHE_NAME = 'peaske-images-v1';
const LOW_RES_CACHE_NAME = 'peaske-low-res-v1';

// Assets to cache immediately on install
const PRECACHE_ASSETS = [
  '/assets/a6400-03871.webp',
  '/assets/a6400-03307.webp',
  '/assets/a6400-03077.webp',
  '/assets/a6400-02423.webp',
  '/assets/a6400-01926.webp',
  '/assets/a6400-01621.webp',
  '/assets/a6400-01439.webp',
  '/assets/a6400-00985.webp',
];

// Install event - precache critical assets
self.addEventListener('install', event => {
  event.waitUntil(
    Promise.all([
      // Cache for general assets
      caches.open(CACHE_NAME).then(cache => {
        return cache.addAll([
          '/',
          '/photos',
          '/service-worker.js',
          // Add CSS and JS files
        ]);
      }),
      // Dedicated cache for images
      caches.open(IMAGE_CACHE_NAME).then(cache => {
        return cache.addAll(PRECACHE_ASSETS);
      }),
      // Low-res versions for slow connections
      caches.open(LOW_RES_CACHE_NAME).then(cache => {
        // Would add low-res versions here
      })
    ])
    .then(() => self.skipWaiting())
  );
});

// Activate event - clean up old caches
self.addEventListener('activate', event => {
  const currentCaches = [CACHE_NAME, IMAGE_CACHE_NAME, LOW_RES_CACHE_NAME];
  event.waitUntil(
    caches.keys().then(cacheNames => {
      return cacheNames.filter(cacheName => !currentCaches.includes(cacheName));
    }).then(cachesToDelete => {
      return Promise.all(cachesToDelete.map(cacheToDelete => {
        return caches.delete(cacheToDelete);
      }));
    }).then(() => self.clients.claim())
  );
});

// Fetch event - serve from cache or network
self.addEventListener('fetch', event => {
  // Only cache GET requests
  if (event.request.method !== 'GET') return;
  
  // Handle image requests specially
  if (event.request.url.match(/\.(jpe?g|png|gif|webp|avif)$/)) {
    event.respondWith(handleImageRequest(event.request));
  } else {
    // For non-image requests
    event.respondWith(
      caches.match(event.request).then(cachedResponse => {
        if (cachedResponse) {
          return cachedResponse;
        }
        return fetch(event.request).then(response => {
          // Don't cache non-successful responses
          if (!response || response.status !== 200) {
            return response;
          }
          
          // Clone the response to cache it and return it
          const responseToCache = response.clone();
          caches.open(CACHE_NAME).then(cache => {
            cache.put(event.request, responseToCache);
          });
          
          return response;
        });
      })
    );
  }
});

// Special handling for image requests
async function handleImageRequest(request) {
  // Try to get from cache first
  const cachedResponse = await caches.match(request);
  if (cachedResponse) {
    return cachedResponse;
  }
  
  // Check network conditions
  let useHighQuality = true;
  if ('connection' in navigator) {
    // Use lower quality images on slow connections
    if (navigator.connection.effectiveType === 'slow-2g' || 
        navigator.connection.effectiveType === '2g' ||
        navigator.connection.saveData) {
      useHighQuality = false;
    }
  }
  
  // If on a slow connection, try to get a lower quality version
  if (!useHighQuality) {
    // Create a URL for the low-res version (e.g., by adding a quality parameter)
    const lowResUrl = request.url.replace(/quality=\d+/, 'quality=30');
    if (lowResUrl !== request.url) {
      try {
        const lowResResponse = await fetch(new Request(lowResUrl));
        if (lowResResponse && lowResResponse.status === 200) {
          // Cache the low-res version
          const responseToCache = lowResResponse.clone();
          const cache = await caches.open(LOW_RES_CACHE_NAME);
          cache.put(request, responseToCache);
          return lowResResponse;
        }
      } catch (error) {
        console.error('Error fetching low-res image:', error);
      }
    }
  }
  
  // If not in cache or couldn't get low-res, fetch from network
  try {
    const response = await fetch(request);
    
    // Cache the new response
    const responseToCache = response.clone();
    const cache = await caches.open(IMAGE_CACHE_NAME);
    cache.put(request, responseToCache);
    
    return response;
  } catch (error) {
    console.error('Error fetching image:', error);
    
    // If offline and not in cache, return a placeholder image
    return new Response(
      `<svg width="400" height="300" xmlns="http://www.w3.org/2000/svg">
        <rect width="400" height="300" fill="#eee"/>
        <text x="50%" y="50%" font-family="sans-serif" font-size="24" text-anchor="middle">Image Unavailable</text>
      </svg>`,
      { 
        status: 503,
        headers: { 'Content-Type': 'image/svg+xml' }
      }
    );
  }
} 