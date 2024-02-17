interface ApiUrls {
  development: string;
  staging: string;
  production: string;
};

const API_URLS: ApiUrls = {
  development: 'https://localhost:8000/api',
  staging: 'https://user-service/api',
  production: 'https://production/api'
};

export default API_URLS[import.meta.env.MODE as keyof ApiUrls];
