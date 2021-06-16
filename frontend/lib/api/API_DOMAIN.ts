export default function getApiDomain(backend_ip: string | undefined): string {
  if (process.env.NODE_ENV === "development") {
    return "http://localhost:8000";
  }
  if (backend_ip) {
    return backend_ip;
  }
  throw new Error("No `BACKEND_IP` environment variable specified");
}
