import { css } from "@emotion/react";
import { useNavigate } from "@remix-run/react";
import axios from "axios";
import { FormEvent, useCallback, useState } from "react";

const containerCss = css`
  display: grid;
  justify-content: center;
  margin-top: 10vh;
`;

const formCss = css`
  width: 250px;

  * {
    display: block;
  }

  input {
    border: 1px solid #d0d0d0;
  }

  > * {
    margin-bottom: 14px;
  }
`;

const fieldLabelCss = css`
  margin-bottom: 2px;
`;

const submitCss = css`
  background-color: #eaeaea;
  padding: 4px 8px;
`;

export default function Login(): React.ReactElement {
  const navigate = useNavigate();

  const [username, setUsername] = useState<string | undefined>();
  const [password, setPassword] = useState<string | undefined>();
  const [error, setError] = useState<string | undefined>();

  const onFormSubmit = useCallback(
    async (e: FormEvent) => {
      e.preventDefault();

      try {
        const res = await axios.post(
          "https://api.howittplains.net/auth/login/",
          {
            username,
            password,
          }
        );

        if (typeof res.data?.token === "string") {
          window.localStorage.setItem("token", res.data.token);
          navigate("/");
        } else {
          setError("Something went wrong, try again");
          setPassword("");
        }
      } catch {
        setError("Something went wrong, try again");
        setPassword("");
      }
    },
    [username, password, navigate]
  );

  return (
    <div css={containerCss}>
      <form css={formCss} onSubmit={onFormSubmit}>
        <h2>Login</h2>
        <div>
          <label css={fieldLabelCss} htmlFor="username">
            Username
          </label>
          <input
            type="text"
            name="username"
            value={username}
            onChange={(e): void => setUsername(e.target.value)}
          ></input>
        </div>
        <div>
          <label css={fieldLabelCss} htmlFor="password">
            Password
          </label>
          <input
            type="password"
            name="password"
            value={password}
            onChange={(e): void => setPassword(e.target.value)}
          ></input>
        </div>
        <input
          css={submitCss}
          type="submit"
          name="submit"
          value="Login"
        ></input>
        {error ? <span>{error}</span> : <></>}
      </form>
    </div>
  );
}
