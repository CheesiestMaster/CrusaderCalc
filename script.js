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
    unitTypeSelect.addEventListener('change', updateValues);
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

    for (const refitType of data.refit_types || []) {
        if (refitType.refitFrom === '*' || refitType.refitFrom === unitTypeSelect.value) {
            const option = document.createElement('option');
            option.value = refitType.name;
            option.textContent = refitType.name;
            refitTypeSelect.appendChild(option);
        }
    }

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

    for (const siege of data.structures || []) {
        const option = document.createElement('option');
        option.value = siege.name;
        option.textContent = siege.name;
        siege1Select.appendChild(option);
    }
    if (siege1Select.querySelector('option[value="None"]')) {
        siege1Select.value = 'None';
    }

    const siege1Img = document.createElement('img');
    siege1Img.id = 'siege1Img';
    siege1Img.style.width = '50px';
    siege1Img.style.height = '50px';
    document.body.appendChild(siege1Img);

    const siege2Select = document.createElement('select');
    siege2Select.id = 'siege2Select';
    siege2Select.addEventListener('change', updateValues);
    document.body.appendChild(siege2Select);

    for (const siege of data.structures || []) {
        const option = document.createElement('option');
        option.value = siege.name;
        option.textContent = siege.name;
        siege2Select.appendChild(option);
    }
    if (siege2Select.querySelector('option[value="None"]')) {
        siege2Select.value = 'None';
    }

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

    for (const pack of data.packs || []) {
        const option = document.createElement('option');
        option.value = pack.name;
        option.textContent = pack.name;
        packsSelect.appendChild(option);
    }
    if (packsSelect.querySelector('option[value="None"]')) {
        packsSelect.value = 'None';
    }

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

    for (const potion of data.potions || []) {
        const option = document.createElement('option');
        option.value = potion.name;
        option.textContent = potion.name;
        potionsSelect.appendChild(option);
    }
    if (potionsSelect.querySelector('option[value="None"]')) {
        potionsSelect.value = 'None';
    }

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

        for (const weapon of data.weapons || []) {
            const option = document.createElement('option');
            option.value = weapon.name;
            option.textContent = weapon.name;
            weaponSelect.appendChild(option);
        }
        if (weaponSelect.querySelector('option[value="None"]')) {
            weaponSelect.value = 'None';
        }

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

        for (const weaponUpgrade of data.weaponUpgrades || []) {
            const option = document.createElement('option');
            option.value = weaponUpgrade.name;
            option.textContent = weaponUpgrade.name;
            weaponUpgradeSelect.appendChild(option);
        }
        if (weaponUpgradeSelect.querySelector('option[value="None"]')) {
            weaponUpgradeSelect.value = 'None';
        }

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

        for (const spell of data.spells || []) {
            const option = document.createElement('option');
            option.value = spell.name;
            option.textContent = spell.name;
            spellSelect.appendChild(option);
        }
        if (spellSelect.querySelector('option[value="None"]')) {
            spellSelect.value = 'None';
        }

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

        for (const structure of data.structures || []) {
            const option = document.createElement('option');
            option.value = structure.name;
            option.textContent = structure.name;
            structureSelect.appendChild(option);
        }
        if (structureSelect.querySelector('option[value="None"]')) {
            structureSelect.value = 'None';
        }

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

        // Calculate total unit cost
        let totalCost = 0;
        
        // Add refit cost
        totalCost += getDataValue('refit_types', refitTypeSelect.value, 'cost');
        
        // Add siege costs
        totalCost += siege1Cost + siege2Cost;
        
        // Add pack cost
        totalCost += packsCost;
        
        // Add potion cost
        totalCost += potionsCost;
        
        // Add weapon costs
        for (let i = 1; i <= 5; i++) {
            const weaponSelect = document.getElementById(`weapon${i}Select`);
            if (weaponSelect) {
                totalCost += getDataValue('weapons', weaponSelect.value, 'cost');
            }
        }
        
        // Add weapon upgrade costs
        for (let i = 1; i <= 5; i++) {
            const weaponUpgradeSelect = document.getElementById(`weaponUpgrade${i}Select`);
            if (weaponUpgradeSelect) {
                totalCost += getDataValue('weaponUpgrades', weaponUpgradeSelect.value, 'cost');
            }
        }
        
        // Add spell costs
        for (let i = 1; i <= 5; i++) {
            const spellSelect = document.getElementById(`spell${i}Select`);
            if (spellSelect) {
                totalCost += getDataValue('spells', spellSelect.value, 'cost');
            }
        }
        
        // Add structure costs
        for (let i = 1; i <= 5; i++) {
            const structureSelect = document.getElementById(`structure${i}Select`);
            if (structureSelect) {
                totalCost += getDataValue('structures', structureSelect.value, 'cost');
            }
        }
        
        // Update unit cost input
        const unitCostInput = document.getElementById('unitCostInput');
        if (unitCostInput) {
            unitCostInput.value = totalCost;
        }

        // Calculate unit gold remaining
        const unitGoldTotal = unitTypeGold + refitTypeGold;
        const unitGoldRemainingInput = document.getElementById('unitGoldRemainingInput');
        if (unitGoldRemainingInput) {
            unitGoldRemainingInput.value = unitGoldTotal;
        }

        // Calculate remaining budget
        const goldBudgetInput = document.getElementById('goldBudgetInput');
        const remainingBudgetInput = document.getElementById('remainingBudgetInput');
        if (goldBudgetInput && remainingBudgetInput) {
            const goldBudget = parseFloat(goldBudgetInput.value) || 0;
            const remainingBudget = goldBudget - totalCost;
            remainingBudgetInput.value = remainingBudget >= 0 ? remainingBudget : 0;
        }
    }

    // Initialize all values on page load
    updateValues();
});