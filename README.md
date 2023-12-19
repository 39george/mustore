DB Requirements:

- If user deletes conversation, delete him from participants, if he is not single interlocutor.
- Restrict sending messages if user is the only reguar user in conversation.
- Allow to change/delete messages in 5 hours from the creating timestamp. And restrict if already was answered.
- When creating user, create user_settings, when deleting user, delete user settings

Current To-Do:

Testing a backend application for user signup and login involves several layers of testing from unit tests to integration and end-to-end tests. Below is a comprehensive list of tests that you should consider writing to ensure the robustness and reliability of your Rust Axum application's authentication flow:

### Unit Tests:

**1. Validation Logic:**
   - Test if invalid email addresses are rejected.
   - Test if weak passwords are rejected according to your security policy.
   - Test if empty fields (username, password, etc.) are rejected.
   - Test password hashing function, ensuring it does not return the same value for the same input.

**2. User Model:**
   - Check if a new user instance can be created with valid attributes.
   - Ensure that user instance cannot be saved without required fields.
   - Ensure that duplicates cannot be created for unique fields like email.

**3. Authentication Logic:**
   - Ensure correct tokens are generated for valid credentials.
   - Ensure invalid credentials do not generate tokens.
   - Test token expiration logic.

**4. Request Handlers:**
   - Mock dependencies and ensure handlers return the correct HTTP status codes and messages for various scenarios.

### Integration Tests:

**1. Database Operations:**
   - Test user creation and retrieval from the database.
   - Verify that the correct errors are returned when operations fail (unique constraint, etc.).

**2. Authentication Workflow:**
   - Test the login flow, ensuring that a successful login returns a token and correct status.
   - Test that a failed login (incorrect password or email) does not return a token.

**3. Session Management:**
   - Test that the session handling works correctly with valid and invalid tokens.
   - Test session expiration and renewal mechanics.

**4. API Routes:**
   - Verify that all user-related endpoints work as expected with valid and invalid data (sign up, log in, log out).
   - Test rate limiting, if implemented, on endpoints.

### End-to-End Tests:

**1. User Signup Flow:**
   - Register with valid credentials and expect success response.
   - Try to register with the same credentials again and expect a failure due to duplication.
   - Register with invalid data (e.g., invalid email, weak password) and expect error responses.

**2. User Login Flow:**
   - Attempt login with a non-existent user and expect failure.
   - Login with correct credentials and expect a success token.
   - Login with incorrect password and expect failure.

**3. Password Resets (if applicable):**
   - Initiate password reset with a valid email and expect success.
   - Try to use the password reset with an invalid token and expect failure.

**4. User Profile Updates:**
   - Test updating user data such as email, password, and profile information, with adequate authentication.

### Security Tests:

**1. Input Sanitization:**
   - Test to ensure that all inputs are sanitized to prevent SQL injection, XSS, etc.

**2. Authentication Bypasses:**
   - Test for common authentication bypass vulnerabilities.

**3. Brute Force Protection:**
   - Test the account lockout mechanism after a number of failed login attempts.

**4. Access Control:**
   - Test that authenticated endpoints do not divulge sensitive data and are not accessible without proper credentials.

**5. Token Security:**
   - Test that tokens cannot be tampered with or forged.
   - Ensure that tokens are stored and transmitted securely (e.g., no tokens in logs, HTTPS-only cookies, etc.).

### Load Tests:

**1. Simulate Concurrent Logins:**
   - Ensure the application can handle a surge of login requests.

**2. Simulate High Signup Rate:**
   - Ensure the database and application handle a high signup rate without deteriorated performance.

**3. Stress Test Authentication System:**
   - Identify breaking points in the authentication system under very high load.

### Miscellaneous Tests:

**1. Dependency Checks:**
   - Verify that the application behaves correctly if a dependent service (like database, email server for password reset) is down.

**2. Configuration and Deployment:**
   - Check if different configurations don’t break the authentication logic (e.g., different environments like testing, staging, production).

**3. Cross-Origin Resource Sharing (CORS):**
   - Ensure that CORS is correctly configured so that only approved origins can make requests.

When writing these tests, it is important to mock external dependencies and to create test doubles for parts of the system not under test. This way, you can isolate each component and ensure the reliability of your tests. Always consider the balance between different types of tests—while unit tests are great for testing small units of code, integration, and end-to-end tests help guarantee that the system works as a whole.

### Future features:
   1. Blog section
   2. Implement rejection of product/service with possibility to correct files that required corrections.

### NOTEBOOK SECTION

I need to get x new songs sorted from new to old keeping in mind likes and listenings count.
