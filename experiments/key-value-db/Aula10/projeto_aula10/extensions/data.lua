local function is_leap_year(year)
    return year % 400 == 0
        or (year % 4 == 0 and year % 100 ~= 0)
end

local function days_in_month(year, month)
    if month == 2 then
        if is_leap_year(year) then
            return 29
        end

        return 28
    end

    if month == 4
        or month == 6
        or month == 9
        or month == 11 then
        return 30
    end

    return 31
end

local function is_valid_date(value)
    if type(value) ~= "string" then
        return false
    end

    if #value ~= 10 then
        return false
    end

    if value:sub(5, 5) ~= "-"
        or value:sub(8, 8) ~= "-" then
        return false
    end

    for i = 1, 10 do
        if i ~= 5 and i ~= 8 then
            if tonumber(value:sub(i, i)) == nil then
                return false
            end
        end
    end

    local year = tonumber(value:sub(1, 4))
    local month = tonumber(value:sub(6, 7))
    local day = tonumber(value:sub(9, 10))

    if month < 1 or month > 12 then
        return false
    end

    local max_day = days_in_month(year, month)

    if day < 1 or day > max_day then
        return false
    end

    return true
end

local function format_date(value)
    return value:sub(9, 10)
        .. "/"
        .. value:sub(6, 7)
        .. "/"
        .. value:sub(1, 4)
end

local function add(key, value)
    if not is_valid_date(value) then
        return {
            success = false,
            error = "Data inválida"
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
        value = format_date(valor)
    }
end

return {
    prefix = "data_",

    operations = {
        ADD = add,
        GET = get
    }
}