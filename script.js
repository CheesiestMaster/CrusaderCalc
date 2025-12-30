document.addEventListener('DOMContentLoaded', async () => {

    const header = document.createElement('h1');
    header.textContent = 'Castles and Crusaders Unit Builder';
    document.body.appendChild(header);

    const goldBudgetLabel = document.createElement('label');
    goldBudgetLabel.textContent = 'Gold Budget ';
    document.body.appendChild(goldBudgetLabel);

    const goldBudgetInput = document.createElement('input');
    goldBudgetInput.id = 'goldBudgetInput';
    goldBudgetInput.type = 'number';
    goldBudgetInput.placeholder = '0';
    goldBudgetInput.addEventListener('input', updateValues)
    document.body.appendChild(goldBudgetInput);

    document.body.appendChild(document.createElement('br'));

    const remainingBudgetLabel = document.createElement('label');
    remainingBudgetLabel.textContent = 'Remaining Budget ';
    document.body.appendChild(remainingBudgetLabel);

    const remainingBudgetInput = document.createElement('input');
    remainingBudgetInput.id = 'remainingBudgetInput';
    remainingBudgetInput.type = 'number';
    remainingBudgetInput.placeholder = '0';
    remainingBudgetInput.disabled = true;
    document.body.appendChild(remainingBudgetInput);

    document.body.appendChild(document.createElement('br'));

    const unitCostLabel = document.createElement('label');
    unitCostLabel.textContent = 'Unit Cost ';
    document.body.appendChild(unitCostLabel);

    const unitCostInput = document.createElement('input');
    unitCostInput.id = 'unitCostInput';
    unitCostInput.type = 'number';
    unitCostInput.placeholder = '0';
    unitCostInput.disabled = true;
    document.body.appendChild(unitCostInput);

    document.body.appendChild(document.createElement('br'));

    const unitGoldRemainingLabel = document.createElement('label');
    unitGoldRemainingLabel.textContent = 'Unit Gold Remaining ';
    document.body.appendChild(unitGoldRemainingLabel);

    const unitGoldRemainingInput = document.createElement('input');
    unitGoldRemainingInput.id = 'unitGoldRemainingInput';
    unitGoldRemainingInput.type = 'number';
    unitGoldRemainingInput.placeholder = '0';
    unitGoldRemainingInput.disabled = true;
    document.body.appendChild(unitGoldRemainingInput);

    document.body.appendChild(document.createElement('hr'));

    let data = {};
    
    // query the server for /data.json
    async function fetchData() {
        try {
            const response = await fetch('data.json');
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            return await response.json();
        } catch (error) {
            console.error('Error fetching data:', error);
            return {};
        }
    }

    data = await fetchData();
    console.log(data);

    const unitTypeLabel = document.createElement('label');
    unitTypeLabel.textContent = 'Unit Type ';
    document.body.appendChild(unitTypeLabel);

    const unitTypeSelect = document.createElement('select');
    unitTypeSelect.id = 'unitTypeSelect';
    document.body.appendChild(unitTypeSelect);

    for (const unitType of data.unit_types || []) {
        const option = document.createElement('option');
        option.value = unitType.name;
        option.textContent = unitType.name;
        unitTypeSelect.appendChild(option);
    }
    if (unitTypeSelect.querySelector('option[value="None"]')) {
        unitTypeSelect.value = 'None';
    }

    const unitTypeImg = document.createElement('img');
    unitTypeImg.id = 'unitTypeImg';
    unitTypeImg.style.width = '50px';
    unitTypeImg.style.height = '50px';
    document.body.appendChild(unitTypeImg);

    const refitTypeLabel = document.createElement('label');
    refitTypeLabel.textContent = ' Refit Type ';
    document.body.appendChild(refitTypeLabel);

    const refitTypeSelect = document.createElement('select');
    refitTypeSelect.id = 'refitTypeSelect';
    refitTypeSelect.addEventListener('change', updateValues);
    document.body.appendChild(refitTypeSelect);

    // Function to update refit dropdown based on selected unit type
    function updateRefitDropdown() {
        // Clear existing options
        refitTypeSelect.innerHTML = '';
        
        const selectedUnitType = unitTypeSelect.value;
        
        // Add refit options that match the selected unit type
        for (const refitType of data.refit_types || []) {
            if (refitType.refitFrom === '*' || refitType.refitFrom === selectedUnitType) {
                const option = document.createElement('option');
                option.value = refitType.name;
                option.textContent = refitType.name;
                refitTypeSelect.appendChild(option);
            }
        }
        
        // Set default to "None" if available
        if (refitTypeSelect.querySelector('option[value="None"]')) {
            refitTypeSelect.value = 'None';
        }
    }

    // Initial population
    updateRefitDropdown();

    // Update refit dropdown and all other dropdowns when unit type or refit type changes
    unitTypeSelect.addEventListener('change', () => {
        updateRefitDropdown();
        updateAllDropdowns();
        updateValues();
    });
    
    refitTypeSelect.addEventListener('change', () => {
        updateAllDropdowns();
        updateValues();
    });

    const refitTypeImg = document.createElement('img');
    refitTypeImg.id = 'refitTypeImg';
    refitTypeImg.style.width = '50px';
    refitTypeImg.style.height = '50px';
    document.body.appendChild(refitTypeImg);

    document.body.appendChild(document.createElement('br'));

    const unitTypeGoldLabel = document.createElement('label');
    unitTypeGoldLabel.textContent = 'Unit Gold ';
    document.body.appendChild(unitTypeGoldLabel);

    const unitTypeGoldInput = document.createElement('input');
    unitTypeGoldInput.id = 'unitTypeGoldInput';
    unitTypeGoldInput.type = 'number';
    unitTypeGoldInput.placeholder = '0';
    unitTypeGoldInput.disabled = true;
    document.body.appendChild(unitTypeGoldInput);

    const refitTypeGoldLabel = document.createElement('label');
    refitTypeGoldLabel.textContent = ' Unit Gold ';
    document.body.appendChild(refitTypeGoldLabel);

    const refitTypeGoldInput = document.createElement('input');
    refitTypeGoldInput.id = 'refitTypeGoldInput';
    refitTypeGoldInput.type = 'number';
    refitTypeGoldInput.placeholder = '0';
    refitTypeGoldInput.disabled = true;
    document.body.appendChild(refitTypeGoldInput);

    document.body.appendChild(document.createElement('br'));

    const siegeLabel = document.createElement('label');
    siegeLabel.textContent = 'Siege ';
    document.body.appendChild(siegeLabel);

    const siege1Select = document.createElement('select');
    siege1Select.id = 'siege1Select';
    siege1Select.addEventListener('change', updateValues);
    document.body.appendChild(siege1Select);

    const siege1Img = document.createElement('img');
    siege1Img.id = 'siege1Img';
    siege1Img.style.width = '50px';
    siege1Img.style.height = '50px';
    document.body.appendChild(siege1Img);

    const siege2Select = document.createElement('select');
    siege2Select.id = 'siege2Select';
    siege2Select.addEventListener('change', updateValues);
    document.body.appendChild(siege2Select);

    const siege2Img = document.createElement('img');
    siege2Img.id = 'siege2Img';
    siege2Img.style.width = '50px';
    siege2Img.style.height = '50px';
    document.body.appendChild(siege2Img);

    const packsLabel = document.createElement('label');
    packsLabel.textContent = ' Pack ';
    document.body.appendChild(packsLabel);

    const packsSelect = document.createElement('select');
    packsSelect.id = 'packsSelect';
    packsSelect.addEventListener('change', updateValues);
    document.body.appendChild(packsSelect);

    const packsImg = document.createElement('img');
    packsImg.id = 'packsImg';
    packsImg.style.width = '50px';
    packsImg.style.height = '50px';
    document.body.appendChild(packsImg);

    const potionsLabel = document.createElement('label');
    potionsLabel.textContent = ' Potion ';
    document.body.appendChild(potionsLabel);

    const potionsSelect = document.createElement('select');
    potionsSelect.id = 'potionsSelect';
    potionsSelect.addEventListener('change', updateValues);
    document.body.appendChild(potionsSelect);

    const potionsImg = document.createElement('img');
    potionsImg.id = 'potionsImg';
    potionsImg.style.width = '50px';
    potionsImg.style.height = '50px';
    document.body.appendChild(potionsImg);

    document.body.appendChild(document.createElement('br'));

    const costLabel1 = document.createElement('label');
    costLabel1.textContent = 'Cost ';
    document.body.appendChild(costLabel1);

    const costInputIds = ['siege1CostInput', 'siege2CostInput', 'packsCostInput', 'potionsCostInput'];
    for (const id of costInputIds) {
        const input = document.createElement('input');
        input.id = id;
        input.type = 'number';
        input.placeholder = '0';
        input.disabled = true;
        document.body.appendChild(input);
    }

    document.body.appendChild(document.createElement('br'));

    // Weapons (5 dropdowns)
    const weaponLabel = document.createElement('label');
    weaponLabel.textContent = 'Weapon ';
    document.body.appendChild(weaponLabel);
    for (let i = 1; i <= 5; i++) {
        const weaponSelect = document.createElement('select');
        weaponSelect.id = `weapon${i}Select`;
        weaponSelect.addEventListener('change', updateValues);
        document.body.appendChild(weaponSelect);

        // Weapons will be populated dynamically

        const weaponImg = document.createElement('img');
        weaponImg.id = `weapon${i}Img`;
        weaponImg.style.width = '50px';
        weaponImg.style.height = '50px';
        document.body.appendChild(weaponImg);
    }
    document.body.appendChild(document.createElement('br'));

    const costLabel2 = document.createElement('label');
    costLabel2.textContent = 'Cost ';
    document.body.appendChild(costLabel2);

    const weaponCostInputIds = [];
    for (let i = 1; i <= 5; i++) {
        weaponCostInputIds.push(`weapon${i}CostInput`);
    }
    for (const id of weaponCostInputIds) {
        const input = document.createElement('input');
        input.id = id;
        input.type = 'number';
        input.placeholder = '0';
        input.disabled = true;
        document.body.appendChild(input);
    }

    document.body.appendChild(document.createElement('br'));

    // Weapon Upgrades (5 dropdowns)
    const weaponUpgradeLabel = document.createElement('label');
    weaponUpgradeLabel.textContent = 'Weapon Upgrade ';
    document.body.appendChild(weaponUpgradeLabel);
    for (let i = 1; i <= 5; i++) {
        const weaponUpgradeSelect = document.createElement('select');
        weaponUpgradeSelect.id = `weaponUpgrade${i}Select`;
        weaponUpgradeSelect.addEventListener('change', updateValues);
        document.body.appendChild(weaponUpgradeSelect);

        // Weapon upgrades will be populated dynamically

        const weaponUpgradeImg = document.createElement('img');
        weaponUpgradeImg.id = `weaponUpgrade${i}Img`;
        weaponUpgradeImg.style.width = '50px';
        weaponUpgradeImg.style.height = '50px';
        document.body.appendChild(weaponUpgradeImg);
    }
    document.body.appendChild(document.createElement('br'));

    const costLabel3 = document.createElement('label');
    costLabel3.textContent = 'Cost ';
    document.body.appendChild(costLabel3);

    const weaponUpgradeCostInputIds = [];
    for (let i = 1; i <= 5; i++) {
        weaponUpgradeCostInputIds.push(`weaponUpgrade${i}CostInput`);
    }
    for (const id of weaponUpgradeCostInputIds) {
        const input = document.createElement('input');
        input.id = id;
        input.type = 'number';
        input.placeholder = '0';
        input.disabled = true;
        document.body.appendChild(input);
    }

    document.body.appendChild(document.createElement('br'));

    // Spells (5 dropdowns)
    const spellLabel = document.createElement('label');
    spellLabel.textContent = 'Spell ';
    document.body.appendChild(spellLabel);
    for (let i = 1; i <= 5; i++) {
        const spellSelect = document.createElement('select');
        spellSelect.id = `spell${i}Select`;
        spellSelect.addEventListener('change', updateValues);
        document.body.appendChild(spellSelect);

        // Spells will be populated dynamically

        const spellImg = document.createElement('img');
        spellImg.id = `spell${i}Img`;
        spellImg.style.width = '50px';
        spellImg.style.height = '50px';
        document.body.appendChild(spellImg);
    }
    document.body.appendChild(document.createElement('br'));

    const costLabel4 = document.createElement('label');
    costLabel4.textContent = 'Cost ';
    document.body.appendChild(costLabel4);

    const spellCostInputIds = [];
    for (let i = 1; i <= 5; i++) {
        spellCostInputIds.push(`spell${i}CostInput`);
    }
    for (const id of spellCostInputIds) {
        const input = document.createElement('input');
        input.id = id;
        input.type = 'number';
        input.placeholder = '0';
        input.disabled = true;
        document.body.appendChild(input);
    }

    document.body.appendChild(document.createElement('br'));

    // Structures (5 dropdowns)
    const structureLabel = document.createElement('label');
    structureLabel.textContent = 'Structure ';
    document.body.appendChild(structureLabel);
    for (let i = 1; i <= 5; i++) {
        const structureSelect = document.createElement('select');
        structureSelect.id = `structure${i}Select`;
        structureSelect.addEventListener('change', updateValues);
        document.body.appendChild(structureSelect);

        // Structures will be populated dynamically

        const structureImg = document.createElement('img');
        structureImg.id = `structure${i}Img`;
        structureImg.style.width = '50px';
        structureImg.style.height = '50px';
        document.body.appendChild(structureImg);
    }
    document.body.appendChild(document.createElement('br'));

    const costLabel5 = document.createElement('label');
    costLabel5.textContent = 'Cost ';
    document.body.appendChild(costLabel5);

    const structureCostInputIds = [];
    for (let i = 1; i <= 5; i++) {
        structureCostInputIds.push(`structure${i}CostInput`);
    }
    for (const id of structureCostInputIds) {
        const input = document.createElement('input');
        input.id = id;
        input.type = 'number';
        input.placeholder = '0';
        input.disabled = true;
        document.body.appendChild(input);
    }

    document.body.appendChild(document.createElement('br'));

    // Helper function to get asset path from data
    function getAssetPath(category, name) {
        if (!name || !data[category]) return '';
        const item = data[category].find(item => item.name === name);
        return item ? item.asset : '';
    }

    // Helper function to get a value from data
    function getDataValue(category, name, attribute) {
        if (!name || !data[category]) return 0;
        const item = data[category].find(item => item.name === name);
        return item && item[attribute] !== undefined ? item[attribute] : 0;
    }

    // Helper function to get active tags (from refit if it has tags, otherwise from unit type)
    function getActiveTags() {
        const refitType = data.refit_types?.find(r => r.name === refitTypeSelect.value);
        if (refitType && refitType.tags && refitType.tags.length > 0) {
            return refitType.tags;
        }
        const unitType = data.unit_types?.find(u => u.name === unitTypeSelect.value);
        return unitType?.tags || [];
    }

    // Helper function to check if an item should be shown based on tags
    function shouldShowItem(item) {
        const activeTags = getActiveTags();
        const unitTypeName = unitTypeSelect.value;
        const refitTypeName = refitTypeSelect.value;

        // Check blockTags first - if any blockTag matches active tags, block the item
        const blockTags = item.blockTags || [];
        for (const blockTag of blockTags) {
            if (activeTags.includes(blockTag)) {
                return false;
            }
        }

        // Check if unit type or refit type is in extraUnits, or if extraUnits contains "*"
        const extraUnits = item.extraUnits || [];
        if (extraUnits.includes('*') || extraUnits.includes(unitTypeName) || extraUnits.includes(refitTypeName)) {
            return true;
        }

        // Check if any allowedTags match active tags
        const allowedTags = item.allowedTags || [];
        for (const allowedTag of allowedTags) {
            if (activeTags.includes(allowedTag)) {
                return true;
            }
        }

        // If no allowedTags specified and no extraUnits match, don't show it
        // (items need explicit permission via allowedTags or extraUnits)
        return false;
    }

    // Function to populate a dropdown with filtered items
    function populateDropdown(selectElement, items, currentValue) {
        selectElement.innerHTML = '';
        for (const item of items) {
            if (shouldShowItem(item)) {
                const option = document.createElement('option');
                option.value = item.name;
                option.textContent = item.name;
                selectElement.appendChild(option);
            }
        }
        // Try to restore previous selection or default to "None"
        if (currentValue && selectElement.querySelector(`option[value="${currentValue}"]`)) {
            selectElement.value = currentValue;
        } else if (selectElement.querySelector('option[value="None"]')) {
            selectElement.value = 'None';
        }
    }

    // Function to update all dropdowns based on current unit/refit selection
    function updateAllDropdowns() {
        // Update siege dropdowns
        const siege1Current = siege1Select.value;
        const siege2Current = siege2Select.value;
        populateDropdown(siege1Select, data.structures || [], siege1Current);
        populateDropdown(siege2Select, data.structures || [], siege2Current);

        // Update packs dropdown
        const packsCurrent = packsSelect.value;
        populateDropdown(packsSelect, data.packs || [], packsCurrent);

        // Update potions dropdown
        const potionsCurrent = potionsSelect.value;
        populateDropdown(potionsSelect, data.potions || [], potionsCurrent);

        // Update weapon dropdowns
        for (let i = 1; i <= 5; i++) {
            const weaponSelect = document.getElementById(`weapon${i}Select`);
            if (weaponSelect) {
                const current = weaponSelect.value;
                populateDropdown(weaponSelect, data.weapons || [], current);
            }
        }

        // Update weapon upgrade dropdowns
        for (let i = 1; i <= 5; i++) {
            const weaponUpgradeSelect = document.getElementById(`weaponUpgrade${i}Select`);
            if (weaponUpgradeSelect) {
                const current = weaponUpgradeSelect.value;
                populateDropdown(weaponUpgradeSelect, data.weaponUpgrades || [], current);
            }
        }

        // Update spell dropdowns
        for (let i = 1; i <= 5; i++) {
            const spellSelect = document.getElementById(`spell${i}Select`);
            if (spellSelect) {
                const current = spellSelect.value;
                populateDropdown(spellSelect, data.spells || [], current);
            }
        }

        // Update structure dropdowns
        for (let i = 1; i <= 5; i++) {
            const structureSelect = document.getElementById(`structure${i}Select`);
            if (structureSelect) {
                const current = structureSelect.value;
                populateDropdown(structureSelect, data.structures || [], current);
            }
        }
    }

    // Function to update all images
    function updateImages() {
        // Unit Type
        const unitTypeImg = document.getElementById('unitTypeImg');
        if (unitTypeImg) {
            const asset = getAssetPath('unit_types', unitTypeSelect.value);
            unitTypeImg.src = asset || '';
        }

        // Refit Type
        const refitTypeImg = document.getElementById('refitTypeImg');
        if (refitTypeImg) {
            const asset = getAssetPath('refit_types', refitTypeSelect.value);
            refitTypeImg.src = asset || '';
        }

        // Siege 1
        const siege1Img = document.getElementById('siege1Img');
        if (siege1Img) {
            const asset = getAssetPath('structures', siege1Select.value);
            siege1Img.src = asset || '';
        }

        // Siege 2
        const siege2Img = document.getElementById('siege2Img');
        if (siege2Img) {
            const asset = getAssetPath('structures', siege2Select.value);
            siege2Img.src = asset || '';
        }

        // Pack
        const packsImg = document.getElementById('packsImg');
        if (packsImg) {
            const asset = getAssetPath('packs', packsSelect.value);
            packsImg.src = asset || '';
        }

        // Potion
        const potionsImg = document.getElementById('potionsImg');
        if (potionsImg) {
            const asset = getAssetPath('potions', potionsSelect.value);
            potionsImg.src = asset || '';
        }

        // Weapons
        for (let i = 1; i <= 5; i++) {
            const weaponImg = document.getElementById(`weapon${i}Img`);
            if (weaponImg) {
                const weaponSelect = document.getElementById(`weapon${i}Select`);
                if (weaponSelect) {
                    const asset = getAssetPath('weapons', weaponSelect.value);
                    weaponImg.src = asset || '';
                }
            }
        }

        // Weapon Upgrades
        for (let i = 1; i <= 5; i++) {
            const weaponUpgradeImg = document.getElementById(`weaponUpgrade${i}Img`);
            if (weaponUpgradeImg) {
                const weaponUpgradeSelect = document.getElementById(`weaponUpgrade${i}Select`);
                if (weaponUpgradeSelect) {
                    const asset = getAssetPath('weaponUpgrades', weaponUpgradeSelect.value);
                    weaponUpgradeImg.src = asset || '';
                }
            }
        }

        // Spells
        for (let i = 1; i <= 5; i++) {
            const spellImg = document.getElementById(`spell${i}Img`);
            if (spellImg) {
                const spellSelect = document.getElementById(`spell${i}Select`);
                if (spellSelect) {
                    const asset = getAssetPath('spells', spellSelect.value);
                    spellImg.src = asset || '';
                }
            }
        }

        // Structures
        for (let i = 1; i <= 5; i++) {
            const structureImg = document.getElementById(`structure${i}Img`);
            if (structureImg) {
                const structureSelect = document.getElementById(`structure${i}Select`);
                if (structureSelect) {
                    const asset = getAssetPath('structures', structureSelect.value);
                    structureImg.src = asset || '';
                }
            }
        }
    }

    function updateValues() {
        // Update images
        updateImages();

        // Update Unit Type Gold
        const unitTypeGold = getDataValue('unit_types', unitTypeSelect.value, 'unitGold');
        const unitTypeGoldInput = document.getElementById('unitTypeGoldInput');
        if (unitTypeGoldInput) {
            unitTypeGoldInput.value = unitTypeGold;
        }

        // Update Refit Type Gold
        const refitTypeGold = getDataValue('refit_types', refitTypeSelect.value, 'unitGold');
        const refitTypeGoldInput = document.getElementById('refitTypeGoldInput');
        if (refitTypeGoldInput) {
            refitTypeGoldInput.value = refitTypeGold;
        }

        // Update Siege costs
        const siege1Cost = getDataValue('structures', siege1Select.value, 'cost');
        const siege1CostInput = document.getElementById('siege1CostInput');
        if (siege1CostInput) {
            siege1CostInput.value = siege1Cost;
        }

        const siege2Cost = getDataValue('structures', siege2Select.value, 'cost');
        const siege2CostInput = document.getElementById('siege2CostInput');
        if (siege2CostInput) {
            siege2CostInput.value = siege2Cost;
        }

        // Update Pack cost
        const packsCost = getDataValue('packs', packsSelect.value, 'cost');
        const packsCostInput = document.getElementById('packsCostInput');
        if (packsCostInput) {
            packsCostInput.value = packsCost;
        }

        // Update Potion cost
        const potionsCost = getDataValue('potions', potionsSelect.value, 'cost');
        const potionsCostInput = document.getElementById('potionsCostInput');
        if (potionsCostInput) {
            potionsCostInput.value = potionsCost;
        }

        // Update Weapon costs
        for (let i = 1; i <= 5; i++) {
            const weaponSelect = document.getElementById(`weapon${i}Select`);
            if (weaponSelect) {
                const weaponCost = getDataValue('weapons', weaponSelect.value, 'cost');
                const weaponCostInput = document.getElementById(`weapon${i}CostInput`);
                if (weaponCostInput) {
                    weaponCostInput.value = weaponCost;
                }
            }
        }

        // Update Weapon Upgrade costs
        for (let i = 1; i <= 5; i++) {
            const weaponUpgradeSelect = document.getElementById(`weaponUpgrade${i}Select`);
            if (weaponUpgradeSelect) {
                const weaponUpgradeCost = getDataValue('weaponUpgrades', weaponUpgradeSelect.value, 'cost');
                const weaponUpgradeCostInput = document.getElementById(`weaponUpgrade${i}CostInput`);
                if (weaponUpgradeCostInput) {
                    weaponUpgradeCostInput.value = weaponUpgradeCost;
                }
            }
        }

        // Update Spell costs
        for (let i = 1; i <= 5; i++) {
            const spellSelect = document.getElementById(`spell${i}Select`);
            if (spellSelect) {
                const spellCost = getDataValue('spells', spellSelect.value, 'cost');
                const spellCostInput = document.getElementById(`spell${i}CostInput`);
                if (spellCostInput) {
                    spellCostInput.value = spellCost;
                }
            }
        }

        // Update Structure costs
        for (let i = 1; i <= 5; i++) {
            const structureSelect = document.getElementById(`structure${i}Select`);
            if (structureSelect) {
                const structureCost = getDataValue('structures', structureSelect.value, 'cost');
                const structureCostInput = document.getElementById(`structure${i}CostInput`);
                if (structureCostInput) {
                    structureCostInput.value = structureCost;
                }
            }
        }

        // Get unit type cost (must be paid from company gold, not unit gold)
        const unitTypeCost = getDataValue('unit_types', unitTypeSelect.value, 'cost');
        
        // Calculate refit cost (must be paid from company gold, not unit gold)
        const refitCost = getDataValue('refit_types', refitTypeSelect.value, 'cost');
        
        // Collect all non-refit costs (can be paid from unit gold)
        // Note: unit type cost is NOT included here as it must be paid from company gold
        const nonRefitCosts = [];
        
        // Siege costs
        if (siege1Cost > 0) nonRefitCosts.push(siege1Cost);
        if (siege2Cost > 0) nonRefitCosts.push(siege2Cost);
        
        // Pack cost
        if (packsCost > 0) nonRefitCosts.push(packsCost);
        
        // Potion cost
        if (potionsCost > 0) nonRefitCosts.push(potionsCost);
        
        // Weapon costs
        for (let i = 1; i <= 5; i++) {
            const weaponSelect = document.getElementById(`weapon${i}Select`);
            if (weaponSelect) {
                const weaponCost = getDataValue('weapons', weaponSelect.value, 'cost');
                if (weaponCost > 0) nonRefitCosts.push(weaponCost);
            }
        }
        
        // Weapon upgrade costs
        for (let i = 1; i <= 5; i++) {
            const weaponUpgradeSelect = document.getElementById(`weaponUpgrade${i}Select`);
            if (weaponUpgradeSelect) {
                const weaponUpgradeCost = getDataValue('weaponUpgrades', weaponUpgradeSelect.value, 'cost');
                if (weaponUpgradeCost > 0) nonRefitCosts.push(weaponUpgradeCost);
            }
        }
        
        // Spell costs
        for (let i = 1; i <= 5; i++) {
            const spellSelect = document.getElementById(`spell${i}Select`);
            if (spellSelect) {
                const spellCost = getDataValue('spells', spellSelect.value, 'cost');
                if (spellCost > 0) nonRefitCosts.push(spellCost);
            }
        }
        
        // Structure costs
        for (let i = 1; i <= 5; i++) {
            const structureSelect = document.getElementById(`structure${i}Select`);
            if (structureSelect) {
                const structureCost = getDataValue('structures', structureSelect.value, 'cost');
                if (structureCost > 0) nonRefitCosts.push(structureCost);
            }
        }
        
        // Sort costs descending to try to pay larger items from unit gold first
        nonRefitCosts.sort((a, b) => b - a);
        
        // Track unit gold pools separately
        // Each purchase must be paid wholly from one pool: base unit gold, refit unit gold, or company gold
        let baseUnitGoldRemaining = unitTypeGold;
        let refitUnitGoldRemaining = refitTypeGold;
        let baseUnitGoldUsed = 0;
        let refitUnitGoldUsed = 0;
        const costsPaidFromCompanyGold = [];
        
        for (const cost of nonRefitCosts) {
            // Try to pay from base unit gold first
            if (baseUnitGoldRemaining >= cost) {
                baseUnitGoldUsed += cost;
                baseUnitGoldRemaining -= cost;
            }
            // If base unit gold isn't enough, try refit unit gold
            else if (refitUnitGoldRemaining >= cost) {
                refitUnitGoldUsed += cost;
                refitUnitGoldRemaining -= cost;
            }
            // If neither unit gold pool has enough, pay from company gold
            else {
                costsPaidFromCompanyGold.push(cost);
            }
        }
        
        // Calculate total unit gold used
        const unitGoldUsed = baseUnitGoldUsed + refitUnitGoldUsed;
        const remainingUnitGold = baseUnitGoldRemaining + refitUnitGoldRemaining;
        
        // Unit type cost and refit cost always come from company gold
        let companyGoldUsed = 0;
        if (unitTypeCost > 0) {
            companyGoldUsed += unitTypeCost;
        }
        if (refitCost > 0) {
            companyGoldUsed += refitCost;
        }
        
        // Add non-refit costs that couldn't be paid from unit gold
        for (const cost of costsPaidFromCompanyGold) {
            companyGoldUsed += cost;
        }
        
        // Calculate total cost
        const totalCost = unitGoldUsed + companyGoldUsed;
        
        // Update unit cost input
        const unitCostInput = document.getElementById('unitCostInput');
        if (unitCostInput) {
            unitCostInput.value = totalCost;
        }

        // Update unit gold remaining
        const unitGoldRemainingInput = document.getElementById('unitGoldRemainingInput');
        if (unitGoldRemainingInput) {
            unitGoldRemainingInput.value = remainingUnitGold;
        }

        // Calculate remaining company gold budget
        const goldBudgetInput = document.getElementById('goldBudgetInput');
        const remainingBudgetInput = document.getElementById('remainingBudgetInput');
        if (goldBudgetInput && remainingBudgetInput) {
            const goldBudget = parseFloat(goldBudgetInput.value) || 0;
            const remainingBudget = goldBudget - companyGoldUsed;
            remainingBudgetInput.value = remainingBudget;
            
            // Add "bad" class if budget is negative
            if (remainingBudget < 0) {
                remainingBudgetInput.classList.add('bad');
            } else {
                remainingBudgetInput.classList.remove('bad');
            }
        }
    }

    // Initialize all dropdowns and values on page load
    updateAllDropdowns();
    updateValues();
});