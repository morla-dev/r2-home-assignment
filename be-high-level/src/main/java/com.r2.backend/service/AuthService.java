package com.r2.backend.service;

import com.r2.backend.model.AuthResult;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

@Service
public class AuthService {

    @Value("${app.auth.password}")
    private String requiredPassword;

    private final TokenService tokenService;

    public AuthService(TokenService tokenService) {
        this.tokenService = tokenService;
    }

    public AuthResult isAuthenticated(String authHeader){
        if (authHeader == null || !authHeader.startsWith("Bearer ")) {
            return AuthResult.failure("Missing or invalid Authorization header");
        }

        String token = authHeader.substring(7);
        if (!tokenService.isValid(token)) {
            return AuthResult.failure("Invalid or expired token");
        }

        return AuthResult.success(token);
    }

    public String login(String email, String password) {
        if (!isValidEmail(email) || !requiredPassword.equals(password)) {
            return null;
        }
        return tokenService.generate();
    }

    public boolean logout(String token) {
        return tokenService.invalidate(token);
    }

    private boolean isValidEmail(String email) {
        return email != null && email.matches("^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$");
    }

}

