import { useCallback, useEffect, useState } from "react";

export default function Login() {
  const [apiKey, setApiKey] = useState<string | undefined>();

  useEffect(() => {
    const storedApiKey = window.localStorage.getItem("apiKey");
    if (storedApiKey) {
      setApiKey(storedApiKey);
    }
  }, [setApiKey]);

  const onFormSubmit = useCallback(() => {
    if (apiKey) {
      window.localStorage.setItem("apiKey", apiKey);
    }
  }, [apiKey]);

  return (
    <form onSubmit={onFormSubmit}>
      <h2>Login</h2>
      <label htmlFor="apiKey">API Key</label>
      <input
        type="text"
        name="apiKey"
        value={apiKey}
        onChange={(e) => setApiKey(e.target.value)}
      ></input>
      <input type="submit" name="submit"></input>
    </form>
  );
}
