#r "https://raw.githubusercontent.com/Emik03/Emik.Morsels/refs/heads/main/Snippets/REPL.csx"

(string Id, int Amount)[] players = [("c", 5), ("e", 1), ("g", 5), ("h", 1), ("l", 5), ("s", 1), ("verity", 2), ("cobble", 3), ("ether", 3)];

IEnumerable<List<(string Id, int Submission)>> Numbers()
{
    for (var c = 0; c <= 5; c++)
        for (var e = 0; e <= 1; e++)
            for (var g = 0; g <= 5; g++)
                for (var h = 0; h <= 1; h++)
                    for (var l = 0; l <= 5; l++)
                        for (var s = 0; s <= 1; s++)
                            for (var verity = 0; verity <= 2; verity++)
                                for (var cobble = 0; cobble <= 3; cobble++)
                                    for (var ether = 0; ether <= 3; ether++)
                                        yield return [("c", c), ("e", e), ("g", g), ("h", h), ("l", l), ("s", s), ("verity", verity), ("cobble", cobble), ("ether", ether)];
}

var answer = 19683
    .For(x =>
    {
        List<(string Id, int Amount)> bronze = new(), silver = new(), gold = new();

        for (var i = 0; i < 9; i++, x /= 3)
            (x % 3 is 0 ? bronze : x % 3 is 1 ? silver : gold).Add(players[i]);

        return (bronze, silver, gold);
    })
    .CartesianProduct(Numbers())
    .Select(x => (bronze: x.First.bronze.ToList(), silver: x.First.silver.ToList(), gold: x.First.gold.ToList(), submissions: x.Second))
    .Select(x =>
    {
        Mutate(x.bronze, x.submissions, 1.4);
        Mutate(x.silver, x.submissions, 1.7);
        Mutate(x.gold, x.submissions, 2);
        x.bronze.AddRange(x.silver);
        x.bronze.AddRange(x.gold);
        return (bronze: x.bronze, x.silver, x.gold, x.submissions);
    })
    .Where(x =>
    {
        foreach (var y in x.bronze)
        {
            var expected = y.Id switch
            {
                "c" => 4,
                "e" => 2,
                "g" => 3,
                "h" => 2,
                "l" => 3,
                "s" => 1,
                _ => 0,
            };

            if (expected is not 0 && y.Amount != expected)
                return false;
        }

        return true;
    })
    .Where(x => x.bronze.Find(x => x.Id == "verity").Amount is 0 || x.bronze.Find(x => x.Id == "cobble").Amount is 0 || x.bronze.Find(x => x.Id == "ether").Amount is 0)
    .Select(x => $"Result: {x.bronze.Where(x => x.Id is "verity" or "cobble" or "ether").Conjoin()}, Silver: {x.silver.Select(x => x.Id).Conjoin()}, Gold: {x.gold.Select(x => x.Id).Conjoin()}, Submissions: {x.submissions.Conjoin()}")
    .FirstOrDefault();

Console.WriteLine(answer);
