import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { css } from "@emotion/react";
import { useNavigate } from "@remix-run/react";
import axios from "axios";
import Cookies from "js-cookie";
import { FormEvent, useCallback, useState } from "react";
import { gql } from "~/__generated__";
import { getApiBaseUrl } from "~/env.client";
import * as Tabs from "@radix-ui/react-tabs";
import { tokens } from "~/styles/tokens";

const containerCss = css`
  display: grid;
  justify-content: center;
  margin-top: 10vh;
`;

const formCss = css`
  width: 300px;

  * {
    display: block;
  }

  input {
    border: 1px solid ${tokens.colors.lightGrey};
    padding: 8px;
    width: 100%;
    margin-bottom: 14px;
    border-radius: 4px;
  }
`;

const fieldLabelCss = css`
  margin-bottom: 2px;
  color: ${tokens.colors.darkGrey};
`;

const submitCss = css`
  background-color: white;
  border: 1px solid ${tokens.colors.lightGrey};
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
  width: 100%;

  &:hover {
    background-color: ${tokens.colors.offWhite};
  }

  &:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }
`;

const errorCss = css`
  color: #ff4444;
  margin-bottom: 14px;
  font-size: 0.9em;
`;

const tabListCss = css`
  display: flex;
  margin-bottom: 20px;
  border-bottom: 1px solid ${tokens.colors.lightGrey};
`;

const tabTriggerCss = css`
  padding: 8px 16px;
  border: none;
  background: none;
  cursor: pointer;
  color: ${tokens.colors.darkGrey};

  &[data-state="active"] {
    color: ${tokens.colors.black};
    border-bottom: 2px solid ${tokens.colors.black};
  }

  &:hover {
    background-color: ${tokens.colors.offWhite};
  }
`;

const tabContentCss = css`
  &[data-state="inactive"] {
    display: none;
  }
`;

const LoginQuery = gql(`
  query LoginViewerInfo {
    viewer {
      id
      profile {
        username
      }
    ...viewerInfo
    }
  }  
`);

export default function Login(): React.ReactElement {
  const navigate = useNavigate();
  const { refetch } = useQuery(LoginQuery);

  // Login state
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [loginError, setLoginError] = useState<string>();

  // Signup state
  const [signupUsername, setSignupUsername] = useState<string>("");
  const [signupEmail, setSignupEmail] = useState<string>("");
  const [signupPassword, setSignupPassword] = useState<string>("");
  const [signupConfirmPassword, setSignupConfirmPassword] =
    useState<string>("");

  const [signupError, setSignupError] = useState<string>();

  // Loading states
  const [isLoading, setIsLoading] = useState(false);

  const onLoginSubmit = useCallback(
    async (e: FormEvent) => {
      e.preventDefault();
      setLoginError(undefined);
      setIsLoading(true);

      try {
        const res = await axios.post(`${getApiBaseUrl()}/auth/login`, {
          username,
          password,
        });

        if (typeof res.data?.token === "string") {
          Cookies.set("token", res.data.token);
          await refetch();
          navigate("/");
        } else {
          setLoginError("Something went wrong, try again");
          setPassword("");
        }
      } catch {
        setLoginError("Invalid username or password");
        setPassword("");
      } finally {
        setIsLoading(false);
      }
    },
    [username, password, navigate, refetch],
  );

  const onSignupSubmit = useCallback(
    async (e: FormEvent) => {
      e.preventDefault();
      setSignupError(undefined);
      setIsLoading(true);

      if (signupPassword !== signupConfirmPassword) {
        setSignupError("Passwords do not match");
        setIsLoading(false);
        return;
      }

      try {
        const res = await axios.post(`${getApiBaseUrl()}/auth/signup`, {
          username: signupUsername,
          email: signupEmail,
          password: signupPassword,
        });

        if (res.data?.token) {
          Cookies.set("token", res.data.token);
          await refetch();
          navigate("/settings");
        } else if (res.data?.error) {
          setSignupError(res.data.error);
        } else {
          setSignupError("Something went wrong, please try again");
        }
      } catch (err) {
        setSignupError("Failed to create account. Please try again.");
      } finally {
        setIsLoading(false);
      }
    },
    [
      signupUsername,
      signupEmail,
      signupPassword,
      signupConfirmPassword,
      navigate,
      refetch,
    ],
  );

  return (
    <div css={containerCss}>
      <Tabs.Root defaultValue="login">
        <Tabs.List css={tabListCss}>
          <Tabs.Trigger value="login" css={tabTriggerCss}>
            Login
          </Tabs.Trigger>
          <Tabs.Trigger value="signup" css={tabTriggerCss}>
            Sign up
          </Tabs.Trigger>
        </Tabs.List>

        <Tabs.Content value="login" css={tabContentCss}>
          <form css={formCss} onSubmit={onLoginSubmit}>
            <div>
              <label css={fieldLabelCss} htmlFor="username">
                Username
              </label>
              <input
                type="text"
                name="username"
                value={username}
                onChange={(e): void => setUsername(e.target.value)}
                autoComplete="username"
                required
              />
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
                autoComplete="current-password"
                required
              />
            </div>
            {loginError && <div css={errorCss}>{loginError}</div>}
            <input
              css={submitCss}
              type="submit"
              value={isLoading ? "Loading..." : "Login"}
              disabled={isLoading}
            />
          </form>
        </Tabs.Content>

        <Tabs.Content value="signup" css={tabContentCss}>
          <form css={formCss} onSubmit={onSignupSubmit}>
            <div>
              <label css={fieldLabelCss} htmlFor="signupUsername">
                Username
              </label>
              <input
                type="text"
                name="signupUsername"
                value={signupUsername}
                onChange={(e): void => setSignupUsername(e.target.value)}
                required
              />
            </div>
            <div>
              <label css={fieldLabelCss} htmlFor="signupEmail">
                Email
              </label>
              <input
                type="email"
                name="signupEmail"
                value={signupEmail}
                onChange={(e): void => setSignupEmail(e.target.value)}
                autoComplete="email"
                required
              />
            </div>
            <div>
              <label css={fieldLabelCss} htmlFor="signupPassword">
                Password
              </label>
              <input
                type="password"
                name="signupPassword"
                value={signupPassword}
                onChange={(e): void => setSignupPassword(e.target.value)}
                autoComplete="new-password"
                required
              />
            </div>
            <div>
              <label css={fieldLabelCss} htmlFor="signupConfirmPassword">
                Confirm Password
              </label>
              <input
                type="password"
                name="signupConfirmPassword"
                value={signupConfirmPassword}
                onChange={(e): void => setSignupConfirmPassword(e.target.value)}
                autoComplete="new-password"
                required
              />
            </div>
            {signupError && <div css={errorCss}>{signupError}</div>}
            <input
              css={submitCss}
              type="submit"
              value={isLoading ? "Creating account..." : "Create account"}
              disabled={isLoading}
            />
          </form>
        </Tabs.Content>
      </Tabs.Root>
    </div>
  );
}
