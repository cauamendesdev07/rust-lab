function executar()
    local valor = get_value("nome")

    if valor == nil then
        return "nome não encontrado"
    end

    local chave = find_key_by_value("Cauã")

    if chave == nil then
        return "valor não encontrado"
    end

    return valor .. " está armazenado em " .. chave
end