DB Requirements:

- If user deletes conversation, delete him from participants, if he is not single interlocutor.
- Restrict sending messages if user is the only reguar user in conversation.
- Allow to change/delete messages in 5 hours from the creating timestamp. And restrict if already was answered.
- When creating user, create user_settings, when deleting user, delete user settings

Current To-Do:

Testing a backend application for user signup and login involves several layers of testing from unit tests to integration and end-to-end tests. Below is a comprehensive list of tests that you should consider writing to ensure the robustness and reliability of your Rust Axum application's authentication flow:

### To-Do from frontender:

   - With user's permissions I need *username*, *likes amount and likes type (meaning on which product is the like)*, *cart amount and type*, *avatar*
   - Delete health check logs from docker compose
   - BUG: wrong port for email confirmation (8000, should be 10000)

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

The query provided does have the potential to be expensive, meaning it could consume significant computational resources and take a while to run. This is due to several factors:

1. **Joins**: The query contains multiple joins, including both inner and left joins, which combine rows from two or more tables. Depending on the size of these tables, the joins can be computationally expensive.

2. **Group By**: The use of `GROUP BY` requires the database engine to aggregate data across potentially large numbers of rows which can be costly in terms of performance.

3. **Aggregation Functions**: The query uses aggregation functions like `COUNT` and `ARRAY_AGG`, which need to process many rows.

4. **Subquery**: The query includes a subquery which is calculated for each row and then used in an outer query.

5. **Order By with Calculation**: The `ORDER BY` clause sorts by a calculated column (`relevance_score`), which means the server must compute this score for all relevant rows before it can sort them. The sorting itself also adds overhead, especially for large datasets.

6. **Non-indexed Calculations**: If the columns used for scoring are not indexed or the operations used cannot take advantage of indexes (e.g., the `GREATEST` function in the `relevance_score`), this could result in a full table scan.

To mitigate some of these costs, you can consider the following:

- **Indexing**: Ensure that columns used in `WHERE`, `JOIN`, and `ORDER BY` clauses are indexed. However, remember that functional expressions used in the `ORDER BY` clause are not directly benefited by indexes unless you store the calculated result in the database and index it.
  
- **Materialized Views**: If the query is run frequently with the same parameters, you might use a materialized view to store a pre-computed version of the query.

- **Incremental Updates**: For frequently updated scores like likes and listenings, consider storing cumulative totals that you update incrementally rather than recalculating from scratch each time.

- **Partitioning**: If the data size is large, consider partitioning the tables by some logical segmentation, which could help manage and query subsets of the data more efficiently.

- **Caching**: For frequently accessed data with scores that do not need to be real-time, caching the results at the application level could significantly reduce database load.

- **Adjust the Scoring Logic**: If real-time precision isn't required, you might calculate relevance scores at regular intervals rather than in real-time with each query.

To truly understand the cost of a query, you can run an `EXPLAIN` or `EXPLAIN ANALYZE` (depending on your SQL database) before the actual query execution. This will show you the query plan and provide insight into which parts of the query are most expensive in terms of computational resources.

### Future features:
   1. Blog section
   2. Implement rejection of product/service with possibility to correct files that required corrections.

### NOTEBOOK SECTION

I need to get x new songs sorted from new to old keeping in mind likes and listenings count.

For example here
```sql
CREATE MATERIALIZED VIEW available_songs AS (
    SELECT
        s.id AS song_id,
        p.name AS product_name,
        u.username as author,
        s.tempo,
        s.key,
        s.duration,
        s.lyric,
        s.lyric,
        s.sex,
        pg.name AS primary_genre,
        sg.name AS secondary_genre,
        p.created_at,
        p.description,
        p.price,
        o.key AS cover_url,
        COUNT(DISTINCT l.id) AS likes, -- Make sure to count distinct records
        COUNT(DISTINCT list.id) AS listenings, -- Same for listenings
        ARRAY_AGG(DISTINCT t.name) FILTER (WHERE pt.products_id = p.id) AS vibes,
        (
            COUNT(DISTINCT l.id) * 1.0 + 
            COUNT(DISTINCT list.id) * 0.5 +
            GREATEST(100 - EXTRACT(DAY FROM CURRENT_DATE - p.created_at), 0) * 1.0
        ) AS relevance_score
    FROM songs s
    JOIN products p ON s.products_id = p.id
    JOIN genres pg ON s.primary_genre = pg.id
    JOIN users u ON p.owner_id = users.id
    LEFT JOIN genres sg ON s.secondary_genre = sg.id
    LEFT JOIN objects o ON o.cover_products_id = s.id
    LEFT JOIN likes l ON l.songs_id = s.id
    LEFT JOIN listenings list ON list.songs_id = s.id
    JOIN products_tags pt ON pt.products_id = p.id
    JOIN tags t ON pt.tags_id = t.id
    GROUP BY s.id, pg.name, sg.name, p.id, o.key
);
```
