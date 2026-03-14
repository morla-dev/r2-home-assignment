package com.r2.backend.controller;

import com.r2.backend.model.LoginRequest;
import com.r2.backend.model.LoginResponse;
import com.r2.backend.model.AuthResult;
import com.r2.backend.service.AuthService;
import com.r2.backend.service.LuckService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.Map;

@RestController
@RequestMapping("/api")
public class ApiController {

    private final AuthService authService;
    private final LuckService luckService;

    public ApiController(AuthService authService,
                         LuckService luckService) {
        this.authService = authService;
        this.luckService = luckService;
    }

    @PostMapping("/login")
    public ResponseEntity<?> login(@RequestBody LoginRequest request) {
        String token = authService.login(request.email(), request.password());
        if (token == null) {
            return ResponseEntity.status(HttpStatus.UNAUTHORIZED).body(Map.of("error", "Invalid email or password"));
        }
        return ResponseEntity.ok(new LoginResponse(token));
    }

    @PostMapping("/logout")
    public ResponseEntity<?> logout(@RequestHeader("Authorization") String authHeader) {
        AuthResult result = authService.isAuthenticated(authHeader);
        if (!result.authenticated()) {
            return ResponseEntity.status(HttpStatus.UNAUTHORIZED).body(result.errorMessage());
        }
        authService.logout(result.token());
        return ResponseEntity.ok("OK");
    }

    @PostMapping("/try_luck")
    public ResponseEntity<?> tryLuck(@RequestHeader("Authorization") String authHeader) {
        AuthResult result = authService.isAuthenticated(authHeader);
        if (!result.authenticated()) {
            return ResponseEntity.status(HttpStatus.UNAUTHORIZED).body(result.errorMessage());
        }
        boolean won = luckService.tryLuck();
        return ResponseEntity.ok(Map.of("win", won));
    }

}
