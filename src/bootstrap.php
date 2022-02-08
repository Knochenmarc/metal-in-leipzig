<?php

declare(strict_types=1);

spl_autoload_register(static function ($className) {
    include_once __DIR__ . '/' . str_replace(['MetalLE\\', '\\'], ['', '/'], $className) . '.php';
});
