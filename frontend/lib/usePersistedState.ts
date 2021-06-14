import { Dispatch, useEffect, useState } from "react";

export default function usePersistedState<T>(
  key: string,
  defaultValue: T
): [T, Dispatch<T>] {
  const [state, setState] = useState(() => {
    let storage = null;
    if (typeof sessionStorage !== "undefined") {
      storage = sessionStorage.getItem(key);
    }
    return storage ? JSON.parse(storage) : defaultValue;
  });
  useEffect(() => {
    if (typeof sessionStorage !== "undefined") {
      sessionStorage.setItem(key, JSON.stringify(state));
    }
  }, [key, state]);
  return [state, setState];
}
