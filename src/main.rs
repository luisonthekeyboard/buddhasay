use rand::Rng;

fn main() {
    let sentences = [
        "Roads come into being as people begin to travel with new purpose in places previously unmarked, each step helping to wear a path on the ground.",
        "I brought the teacup into my mouth and the previous night dissolved into oblivion.",
        "Monastic discipline consists in the scrupulous observance of those rules and doing so it, itself, nothing less than the Dhamma.",
        "Discipline is found in everyday practice of the rules of Buddhism.",
        "Eating is not done to satisfy hunger or appetite, but to carry out the teachings of the Buddha. The art of eating is itself a Buddhist discipline.",
        "There must be a complete departure from human desires.",
        "To cleanse the body and the mind is also to cleanse the world around oneself. The face should not be washed because it«s dirty, or left unwashed because it's not.",
        "The things we find most pleasant pass the most quickly.",
        "Eating is not a question of hunger or satiety, or of food tasting good or bad. The point lays in the act of eating itself. Eating is Buddhism.",
        "Pour all your physical and mental strength into the act of eating.",
        "When your meal is over, stop wanting more.",
        "To suffer for your own sake is a fine thing. Go on suffering like this.",
        "Take your inspiration from those people who, long ago, went to live in far-off mountains and practice ascetic discipline. They not only left the world behind, but cut all ties with it.",
        "If you are swayed by things of the world, how regrettable that is. Things of the world are fleeting and impermanent.",
        "Do not waste a moment.",
        "See to your own enlightenment.",
        "Write the rules of Buddhism on your flesh and bones. Let it be your desire to spend your life peacefully in the path of Buddhism.",
        "Silence is the norm.",
        "Take time for self-reflection. Look back on what you do each day, state the mistakes you made, get the slate wiped clean. Do this every night.",
        "Washing rice, eating pickles and cooking radish are not only culinary arts but also forms of ascetic discipline.",
        "Time is not divided in seven day weeks but is marked off by days ending in 4 and 9. Begin these days one hour early than usual. Shave your face, trim your nails and do personal tasks.",
        "When desires are cut off, the true self appears.",
        "Hair that keeps growing as long as we live, shave it as we will, resembles human desires that know no end.",
        "Buddhist discipline is not a staircase or a means of getting somewhere; it is rather about the successive moments of life, of existence itself.",
        "To observe your breath, to be aware of it, is like being a spectator of yourself.",
        "One should maintain a way-seeking mind, make adjustments in accord to the occasion and see that food is received with ease. Use ingredients appropriate to the season and cooking techniques appropriate to the ingredients.",
        "There is a Buddhist term, \"fragance learning\", which means a kind of unintentional absorption. Just as passing an incense burner can imbue clothing with france, so we are affected unconsciously by the atmosphere of a place, just by happening to be there.",
        "During your practice, you should follow your schedule to the letter - becoming the schedule. Just like following the Dhamma, to follow a schedule is to become a schedule.",
        "An integral part of Buddhist life, no less important than sitting itself, is manual labour. For Buddhists, work has important spiritual value and is integral to the life of discipline.",
        "Sitting is done morning and night.",
        "Cleaning isn't done on special days or special places - it takes place energetically every single day, whether or not there's any dirt to speak of.",
        "Weeding is not done in an attempt to get rid of all the weeds. It's natural that weeds grow and people pull them; it's equally natural that weeds grow back. The point is not to get rid of weeds once and for all, but to carry out the simple repetitive action of pulling them.",
        "Leaves fall and people rake them, people rake them and leaves fall again.",
        "Seasons all come in their turn.",
        "A remarkable bond with the Buddha has brought you to this place. You must give up all thought of self and devote yourself single-mindedly to the pursuit of truth.",
        "You must walk slowly and quietly. Walk as if you're standing still.",
        "It is said that the Buddha attained enlightenment on December 8th. Every year, the first week of December is devoted exclusively to sitting.",
        "When sitting, the person assumes a certain form. To assume this form is to become perfectly one with it, removing all fetters and ego - to be unselfconsciously present in the moment, like air.",
        "You should sit for yourself and persist sitting to the very end.",
        "Devoting yourself to sitting, getting used to sitting and conquering the painn of sitting are equally pointless. The only point of sitting is to accept unconditionally each moment as it occurs.",
        "Freedom in Buddhism means liberation from self interest. Liberation not from any external circumstance but from desire.",
        "Days are relentless in their sameness.",
        "By contemplating life as it is, stripped of all extraneous added value, you can let go of a myriad of things that gnaw at your mind.",
        "Discipline also exists when all restrictions are taken away. Then you face a clear choice: whether to stretch out and idle around or stay committed to your initial purpose.",
        "Solely pursue the truth by practising Buddhism. Don't forget that commitment is for a lifetime.",
        "Nothing lasts, nothing is finished, nothing is perfect."
    ];

    let sentence = rand::thread_rng().gen_range(0, sentences.len());

    println!("{}",sentences[sentence]);
}
