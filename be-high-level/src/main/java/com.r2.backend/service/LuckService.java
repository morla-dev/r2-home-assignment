package com.r2.backend.service;

import org.springframework.stereotype.Service;

import java.time.LocalDate;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicReference;

@Service
public class LuckService {

    private static final int DAILY_WIN_THRESHOLD = 30;
    private static final double BASE_WIN_CHANCE = 0.7;
    private static final double REDUCED_WIN_CHANCE = 0.4;

    private final AtomicInteger dailyWins = new AtomicInteger(0);
    private final AtomicReference<LocalDate> winCountDate = new AtomicReference<>(LocalDate.now());

    public boolean tryLuck() {
        resetDailyWinsIfNewDay();
        double chance = dailyWins.get() >= DAILY_WIN_THRESHOLD ? REDUCED_WIN_CHANCE : BASE_WIN_CHANCE;
        boolean won = Math.random() < chance;
        if (won) {
            dailyWins.incrementAndGet();
        }
        return won;
    }

    private void resetDailyWinsIfNewDay() {
        LocalDate today = LocalDate.now();
        LocalDate tracked = winCountDate.get();
        if (!today.equals(tracked) && winCountDate.compareAndSet(tracked, today)) {
            dailyWins.set(0);
        }
    }
}
