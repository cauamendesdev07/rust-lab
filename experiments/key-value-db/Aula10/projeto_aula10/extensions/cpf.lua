local function is_digit_string(value)
    if type(value) ~= "string" then
        return false
    end

    for i = 1, #value do
        if tonumber(value:sub(i, i)) == nil then
            return false
        end
    end

    return true
end

local function has_all_equal_digits(value)
    local first = value:sub(1, 1)

    for i = 2, #value do
        if value:sub(i, i) ~= first then
            return false
        end
    end

    return true
end

local function calculate_digit(value, start_weight)
    local sum = 0
    local weight = start_weight

    for i = 1, #value do
        local digit = tonumber(value:sub(i, i))

        sum = sum + digit * weight
        weight = weight - 1
    end

    local remainder = sum % 11

    if remainder < 2 then
        return 0
    end

    return 11 - remainder
end

local function is_valid_cpf(cpf)
    if type(cpf) ~= "string" then
        return false
    end

    if #cpf ~= 11 then
        return false
    end

    if not is_digit_string(cpf) then
        return false
    end

    if has_all_equal_digits(cpf) then
        return false
    end

    local first_nine = cpf:sub(1, 9)

    local first_digit = calculate_digit(
        first_nine,
        10
    )

    if first_digit ~= tonumber(cpf:sub(10, 10)) then
        return false
    end

    local first_ten = cpf:sub(1, 10)

    local second_digit = calculate_digit(
        first_ten,
        11
    )

    if second_digit ~= tonumber(cpf:sub(11, 11)) then
        return false
    end

    return true
end

local function format_cpf(cpf)
    return cpf:sub(1, 3)
        .. "."
        .. cpf:sub(4, 6)
        .. "."
        .. cpf:sub(7, 9)
        .. "-"
        .. cpf:sub(10, 11)
end

local function add(key, value)
    if not is_valid_cpf(value) then
        return {
            success = false,
            error = "CPF inválido"
        }
    end

    local existing_key = find_key_by_value(value)

    if existing_key ~= nil and existing_key ~= key then
        return {
            success = false,
            error = "CPF já utilizado pela chave " .. existing_key
        }
    end

    return {
        success = true
    }
end

local function get(key)
    local valor = get_value(key)

    if valor == nil then
        return {
            success = false,
            error = "Chave não encontrada"
        }
    end

    return {
        success = true,
        value = format_cpf(valor)
    }
end

return {
    prefix = "cpf_",

    operations = {
        ADD = add,
        GET = get
    }
}