const API_DOMAIN =
  process.env.NODE_ENV === "development"
    ? "http://localhost:8000"
    : "http://localhost:8000";

export default API_DOMAIN;
