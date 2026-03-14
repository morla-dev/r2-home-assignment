package com.r2.backend.service;

import org.springframework.stereotype.Service;

import java.util.Set;
import java.util.UUID;
import java.util.concurrent.ConcurrentHashMap;

@Service
public class TokenService {

    private final Set<String> activeTokens = ConcurrentHashMap.newKeySet();

    public String generate() {
        String token = UUID.randomUUID().toString();
        activeTokens.add(token);
        return token;
    }

    public boolean isValid(String token) {
        return token != null && activeTokens.contains(token);
    }

    public boolean invalidate(String token) {
        return activeTokens.remove(token);
    }
}
