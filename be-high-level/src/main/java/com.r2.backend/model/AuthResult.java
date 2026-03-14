package com.r2.backend.model;

public record AuthResult(boolean authenticated, String token, String errorMessage) {

    public static AuthResult success(String token) {
        return new AuthResult(true, token, null);
    }

    public static AuthResult failure(String errorMessage) {
        return new AuthResult(false, null, errorMessage);
    }
}