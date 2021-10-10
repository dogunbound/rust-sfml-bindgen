extern "C" void sfRenderWindow_clear(sfRenderWindow *renderWindow, sfColor color) {
    sf::Color SFMLColor(color.r, color.g, color.b, color.a);

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->clear(SFMLColor);
}

extern "C" void sfRenderWindow_setView(sfRenderWindow *renderWindow, const sfView *view) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setView(*reinterpret_cast<const sf::View *>(view));
}

extern "C" const sfView *sfRenderWindow_getView(const sfRenderWindow *renderWindow) {
    return reinterpret_cast<const sfView *>(&reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getView());
}

extern "C" void sfRenderWindow_drawPrimitives(sfRenderWindow *renderWindow,
                                              const sfVertex *vertices, size_t vertexCount,
                                              sfPrimitiveType type, const sf::RenderStates *states) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(reinterpret_cast<const sf::Vertex *>(vertices), vertexCount, static_cast<sf::PrimitiveType>(type), *states);
}