import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { css } from "@emotion/react";
import { useNavigate } from "@remix-run/react";
import axios from "axios";
import Cookies from "js-cookie";
import { useCallback, useState } from "react";
import { useForm } from "react-hook-form";
import { gql } from "~/__generated__";
import { getApiBaseUrl } from "~/env.client";
import * as Tabs from "@radix-ui/react-tabs";
import { tokens } from "~/styles/tokens";
import { tabsListStyles, tabTriggerStyles } from "~/components/ui/Tabs";

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

interface LoginFormInputs {
  username: string;
  password: string;
}

interface SignupFormInputs {
  username: string;
  email: string;
  password: string;
  confirmPassword: string;
}

export default function Login(): React.ReactElement {
  const navigate = useNavigate();
  const { refetch } = useQuery(LoginQuery);
  const [isLoading, setIsLoading] = useState(false);
  const [loginError, setLoginError] = useState<string>();
  const [signupError, setSignupError] = useState<string>();

  const {
    register: loginRegister,
    handleSubmit: handleLoginSubmit,
    formState: { errors: loginErrors },
    reset: resetLoginForm,
  } = useForm<LoginFormInputs>();

  const {
    register: signupRegister,
    handleSubmit: handleSignupSubmit,
    formState: { errors: signupErrors },
    watch,
  } = useForm<SignupFormInputs>();

  const onLoginSubmit = useCallback(
    async (data: LoginFormInputs) => {
      setLoginError(undefined);
      setIsLoading(true);

      try {
        const res = await axios.post(`${getApiBaseUrl()}/auth/login`, {
          username: data.username,
          password: data.password,
        });

        if (typeof res.data?.token === "string") {
          Cookies.set("token", res.data.token);
          await refetch();
          navigate("/");
        } else {
          setLoginError("Something went wrong, try again");
          resetLoginForm({ password: "" });
        }
      } catch {
        setLoginError("Invalid username or password");
        resetLoginForm({ password: "" });
      } finally {
        setIsLoading(false);
      }
    },
    [navigate, refetch, resetLoginForm],
  );

  const onSignupSubmit = useCallback(
    async (data: SignupFormInputs) => {
      setSignupError(undefined);
      setIsLoading(true);

      try {
        const res = await axios.post(`${getApiBaseUrl()}/auth/signup`, {
          username: data.username,
          email: data.email,
          password: data.password,
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
    [navigate, refetch],
  );

  return (
    <div css={containerCss}>
      <Tabs.Root defaultValue="login">
        <Tabs.List css={tabsListStyles}>
          <Tabs.Trigger value="login" css={tabTriggerStyles}>
            Login
          </Tabs.Trigger>
          <Tabs.Trigger value="signup" css={tabTriggerStyles}>
            Sign up
          </Tabs.Trigger>
        </Tabs.List>

        <Tabs.Content value="login">
          <form css={formCss} onSubmit={handleLoginSubmit(onLoginSubmit)}>
            <div>
              <label css={fieldLabelCss} htmlFor="username">
                Username
              </label>
              <input
                type="text"
                {...loginRegister("username", {
                  required: "Username is required",
                })}
                autoComplete="username"
              />
              {loginErrors.username && (
                <div css={errorCss}>{loginErrors.username.message}</div>
              )}
            </div>
            <div>
              <label css={fieldLabelCss} htmlFor="password">
                Password
              </label>
              <input
                type="password"
                {...loginRegister("password", {
                  required: "Password is required",
                })}
                autoComplete="current-password"
              />
              {loginErrors.password && (
                <div css={errorCss}>{loginErrors.password.message}</div>
              )}
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

        <Tabs.Content value="signup">
          <form css={formCss} onSubmit={handleSignupSubmit(onSignupSubmit)}>
            <div>
              <label css={fieldLabelCss} htmlFor="signupUsername">
                Username
              </label>
              <input
                type="text"
                {...signupRegister("username", {
                  required: "Username is required",
                })}
              />
              {signupErrors.username && (
                <div css={errorCss}>{signupErrors.username.message}</div>
              )}
            </div>
            <div>
              <label css={fieldLabelCss} htmlFor="signupEmail">
                Email
              </label>
              <input
                type="email"
                {...signupRegister("email", {
                  required: "Email is required",
                  pattern: {
                    value: /^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$/i,
                    message: "Invalid email address",
                  },
                })}
                autoComplete="email"
              />
              {signupErrors.email && (
                <div css={errorCss}>{signupErrors.email.message}</div>
              )}
            </div>
            <div>
              <label css={fieldLabelCss} htmlFor="signupPassword">
                Password
              </label>
              <input
                type="password"
                {...signupRegister("password", {
                  required: "Password is required",
                })}
                autoComplete="new-password"
              />
              {signupErrors.password && (
                <div css={errorCss}>{signupErrors.password.message}</div>
              )}
            </div>
            <div>
              <label css={fieldLabelCss} htmlFor="signupConfirmPassword">
                Confirm Password
              </label>
              <input
                type="password"
                {...signupRegister("confirmPassword", {
                  required: "Please confirm your password",
                  validate: (value) =>
                    value === watch("password") || "Passwords do not match",
                })}
                autoComplete="new-password"
              />
              {signupErrors.confirmPassword && (
                <div css={errorCss}>{signupErrors.confirmPassword.message}</div>
              )}
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
