"use client";

import { useState } from "react";

interface BlinkResponse {
  id: string;
  action_url: string;
}

type BlinkType = "donation" | "payment" | "vote";

export default function Home() {
  const [blinkType, setBlinkType] = useState<BlinkType>("donation");
  const [formData, setFormData] = useState({
    title: "",
    description: "",
    icon_url: "",
    label: "",
    wallet_address: "",
    amount: "0.1",
    options: "Pizza, Burger, Salad"
  });

  const [result, setResult] = useState<BlinkResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    let config = {};
    if (blinkType === "donation") {
      config = { amount: parseFloat(formData.amount) };
    } else if (blinkType === "vote") {
      config = { options: formData.options.split(",").map((s) => s.trim()) };
    }

    try {
      const response = await fetch(
        `${process.env.NEXT_PUBLIC_API_URL}/api/blinks`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            title: formData.title,
            icon_url: formData.icon_url,
            description: formData.description,
            label: formData.label,
            wallet_address: formData.wallet_address,
            type: blinkType,
            config: config,
          }),
        }
      );

      if (!response.ok) {
        throw new Error("Failed to create blink");
      }

      const data: BlinkResponse = await response.json();
      setResult(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Something went wrong");
    } finally {
      setLoading(false);
    }
  };

  const copyToClipboard = async (text: string) => {
    if (navigator.clipboard) {
      await navigator.clipboard.writeText(text);
      alert("Copied!");
    }
  };

  return (
    <div className="min-h-screen solana-gradient-bg text-white font-sans selection:bg-purple-500/30 relative overflow-hidden">
      {/* Floating Orbs */}
      <div className="solana-orb solana-orb-purple w-96 h-96 -top-48 -left-48" />
      <div className="solana-orb solana-orb-teal w-80 h-80 top-1/3 -right-40" style={{ animationDelay: '-5s' }} />
      <div className="solana-orb solana-orb-blue w-72 h-72 bottom-20 left-1/4" style={{ animationDelay: '-10s' }} />
      <div className="solana-orb solana-orb-purple w-64 h-64 -bottom-32 right-1/3" style={{ animationDelay: '-7s' }} />

      <div className="absolute top-6 right-6 z-20">
        <a
          href="https://github.com/bakayu/blinkzero"
          target="_blank"
          rel="noopener noreferrer"
          className="flex items-center gap-2 px-4 py-2 bg-black/60 hover:bg-black/80 backdrop-blur-sm border border-gray-800/50 rounded-lg transition-all"
        >
          <svg
            className="w-5 h-5"
            fill="currentColor"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <path
              fillRule="evenodd"
              d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
              clipRule="evenodd"
            />
          </svg>
          <span className="text-sm font-medium">GitHub</span>
        </a>
      </div>

      {/* Main Content */}
      <div className="relative z-10 max-w-6xl mx-auto p-6 grid lg:grid-cols-2 gap-12 pt-20">
        {/* LEFT COLUMN: Form */}
        <div className="space-y-8">
          <div className="space-y-2">
            <h1 className="text-5xl font-bold bg-gradient-to-r from-[#7c35d9] via-[#0fc07a] to-[#00a8cc] bg-clip-text text-transparent">
              BlinkZero
            </h1>
            <p className="text-gray-500 text-lg">
              The No-Code Builder for Solana Actions (Blinks).
            </p>
          </div>

          {!result ? (
            <form onSubmit={handleSubmit} className="space-y-6">
              {/* Type Selection */}
              <div className="grid grid-cols-3 gap-2 p-1 bg-black/60 backdrop-blur-sm rounded-xl border border-gray-800/50">
                {(["donation", "payment", "vote"] as BlinkType[]).map((t) => (
                  <button
                    key={t}
                    type="button"
                    onClick={() => setBlinkType(t)}
                    className={`py-2 rounded-lg capitalize font-medium transition-all ${blinkType === t
                      ? "bg-[#6b2cc2] text-white shadow-lg"
                      : "text-gray-500 hover:text-white"
                      }`}
                  >
                    {t}
                  </button>
                ))}
              </div>

              {/* Core Fields */}
              <div className="space-y-4">
                <div>
                  <label className="text-xs font-semibold text-gray-600 uppercase tracking-wider">
                    Title
                  </label>
                  <input
                    required
                    placeholder="e.g. Save the Rainforest"
                    className="w-full mt-1 bg-black/60 backdrop-blur-sm border border-gray-800/50 rounded-lg px-4 py-3 focus:ring-2 focus:ring-[#7c35d9] focus:border-transparent outline-none transition-all placeholder:text-gray-700"
                    value={formData.title}
                    onChange={(e) =>
                      setFormData({ ...formData, title: e.target.value })
                    }
                  />
                </div>

                <div>
                  <label className="text-xs font-semibold text-gray-600 uppercase tracking-wider">
                    Description
                  </label>
                  <textarea
                    required
                    placeholder="Explain what this action does..."
                    rows={3}
                    className="w-full mt-1 bg-black/60 backdrop-blur-sm border border-gray-800/50 rounded-lg px-4 py-3 focus:ring-2 focus:ring-[#7c35d9] focus:border-transparent outline-none transition-all resize-none placeholder:text-gray-700"
                    value={formData.description}
                    onChange={(e) =>
                      setFormData({ ...formData, description: e.target.value })
                    }
                  />
                </div>

                <div>
                  <label className="text-xs font-semibold text-gray-600 uppercase tracking-wider">
                    Image URL
                  </label>
                  <input
                    required
                    type="url"
                    placeholder="https://..."
                    className="w-full mt-1 bg-black/60 backdrop-blur-sm border border-gray-800/50 rounded-lg px-4 py-3 focus:ring-2 focus:ring-[#7c35d9] focus:border-transparent outline-none transition-all placeholder:text-gray-700"
                    value={formData.icon_url}
                    onChange={(e) =>
                      setFormData({ ...formData, icon_url: e.target.value })
                    }
                  />
                </div>

                {/* Dynamic Fields based on Type */}
                <div className="p-4 bg-black/50 backdrop-blur-sm border border-gray-800/50 rounded-lg space-y-4">
                  {blinkType === "donation" && (
                    <div>
                      <label className="text-xs font-semibold text-[#0fc07a] uppercase tracking-wider">
                        Amount (SOL)
                      </label>
                      <input
                        type="number"
                        step="0.01"
                        required
                        className="w-full mt-1 bg-black/60 border border-gray-700/50 rounded-lg px-4 py-3 focus:ring-2 focus:ring-[#0fc07a] outline-none"
                        value={formData.amount}
                        onChange={(e) =>
                          setFormData({ ...formData, amount: e.target.value })
                        }
                      />
                    </div>
                  )}

                  {blinkType === "vote" && (
                    <div>
                      <label className="text-xs font-semibold text-[#0fc07a] uppercase tracking-wider">
                        Voting Options
                      </label>
                      <input
                        type="text"
                        placeholder="Option 1, Option 2, Option 3"
                        required
                        className="w-full mt-1 bg-black/60 border border-gray-700/50 rounded-lg px-4 py-3 focus:ring-2 focus:ring-[#0fc07a] outline-none"
                        value={formData.options}
                        onChange={(e) =>
                          setFormData({ ...formData, options: e.target.value })
                        }
                      />
                      <p className="text-xs text-gray-600 mt-1">
                        Comma separated list of buttons.
                      </p>
                    </div>
                  )}

                  {blinkType === "payment" && (
                    <div className="text-sm text-gray-500">
                      NOTE: Users will be able to enter any custom SOL amount.
                    </div>
                  )}
                </div>

                <div>
                  <label className="text-xs font-semibold text-gray-600 uppercase tracking-wider">
                    Wallet Address (Receiver)
                  </label>
                  <input
                    required
                    placeholder="Solana Wallet Address"
                    className="w-full mt-1 bg-black/60 backdrop-blur-sm border border-gray-800/50 rounded-lg px-4 py-3 font-mono text-sm focus:ring-2 focus:ring-[#7c35d9] outline-none placeholder:text-gray-700"
                    value={formData.wallet_address}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        wallet_address: e.target.value,
                      })
                    }
                  />
                </div>

                <div>
                  <label className="text-xs font-semibold text-gray-600 uppercase tracking-wider">
                    Button Label
                  </label>
                  <input
                    required
                    placeholder={
                      blinkType === "vote" ? "Vote (Prefix)" : "Donate"
                    }
                    className="w-full mt-1 bg-black/60 backdrop-blur-sm border border-gray-800/50 rounded-lg px-4 py-3 focus:ring-2 focus:ring-[#7c35d9] outline-none placeholder:text-gray-700"
                    value={formData.label}
                    onChange={(e) =>
                      setFormData({ ...formData, label: e.target.value })
                    }
                  />
                </div>
              </div>

              {error && (
                <div className="p-4 bg-red-900/30 border border-red-900/50 text-red-400 rounded-lg text-sm backdrop-blur-sm">
                  {error}
                </div>
              )}

              <button
                type="submit"
                disabled={loading}
                className="w-full py-4 bg-[#6b2cc2] text-white font-bold rounded-xl hover:bg-[#5a25a8] transition-all active:scale-[0.98] disabled:opacity-50"
              >
                {loading ? "Deploying..." : "Create Blink"}
              </button>
            </form>
          ) : (
            <div className="space-y-6">
              <div className="p-6 bg-[#0fc07a]/10 border border-[#0fc07a]/30 rounded-xl backdrop-blur-sm">
                <h3 className="text-xl font-bold text-[#0fc07a] mb-2">
                  Blink Deployed!
                </h3>
                <p className="text-gray-500 mb-4">
                  Your action is live on the Solana network (via BlinkZero).
                </p>
                <div className="flex gap-2">
                  <input
                    readOnly
                    value={result.action_url}
                    className="flex-1 bg-black/60 border border-gray-800/50 rounded-lg px-4 py-3 font-mono text-sm text-gray-300"
                  />
                  <button
                    onClick={() => copyToClipboard(result.action_url)}
                    className="px-6 bg-gray-800/60 hover:bg-gray-700/60 rounded-lg font-semibold transition-colors"
                  >
                    Copy
                  </button>
                </div>
              </div>
              <div className="flex gap-4">
                <a
                  href={`https://dial.to/?action=solana-action:${encodeURIComponent(
                    result.action_url
                  )}`}
                  target="_blank"
                  rel="noreferrer"
                  className="flex-1 py-3 text-center bg-[#6b2cc2] hover:bg-[#5a25a8] rounded-xl font-bold transition-all"
                >
                  Test on Dial.to
                </a>
                <button
                  onClick={() => setResult(null)}
                  className="flex-1 py-3 text-center bg-gray-800/60 hover:bg-gray-700/60 rounded-xl font-bold transition-colors backdrop-blur-sm"
                >
                  Create Another
                </button>
              </div>
            </div>
          )}
        </div>

        {/* RIGHT COLUMN: Live Preview - Sticky */}
        <div className="hidden lg:block relative">
          <div className="sticky top-8">
            <div className="max-w-sm mx-auto">
              <h2 className="text-sm font-semibold text-gray-600 uppercase tracking-wider mb-4">
                Live Preview
              </h2>

              {/* Mock Blink Card */}
              <div className="bg-[#141414]/90 backdrop-blur-sm rounded-2xl overflow-hidden shadow-2xl border border-gray-800/50 w-full">
                <div className="aspect-video w-full bg-gray-900 relative">
                  {formData.icon_url ? (
                    <img
                      src={formData.icon_url}
                      alt="Preview"
                      className="w-full h-full object-cover"
                      onError={(e) =>
                      (e.currentTarget.src =
                        "https://placehold.co/600x400/141414/666?text=Image+Error")
                      }
                    />
                  ) : (
                    <div className="w-full h-full flex items-center justify-center text-gray-700">
                      No Image
                    </div>
                  )}
                </div>
                <div className="p-5 space-y-3">
                  <h3 className="text-white font-bold text-lg">
                    {formData.title || "Your Title Here"}
                  </h3>
                  <p className="text-gray-500 text-sm leading-relaxed">
                    {formData.description || "Your description will appear here."}
                  </p>

                  {/* Dynamic Buttons Preview */}
                  <div className="pt-2 space-y-2">
                    {blinkType === "donation" && (
                      <button className="w-full py-2.5 bg-[#6b2cc2] hover:bg-[#5a25a8] text-white rounded-lg font-semibold text-sm transition-colors">
                        {formData.label || "Donate"} {formData.amount} SOL
                      </button>
                    )}

                    {blinkType === "payment" && (
                      <div className="flex gap-2">
                        <input
                          placeholder="Enter amount..."
                          disabled
                          className="w-full px-3 py-2 bg-black/80 border border-gray-700 rounded-lg text-sm"
                        />
                        <button className="px-4 py-2 bg-[#6b2cc2] hover:bg-[#5a25a8] text-white rounded-lg font-semibold text-sm transition-colors">
                          {formData.label || "Pay"}
                        </button>
                      </div>
                    )}

                    {blinkType === "vote" && (
                      <div className="grid grid-cols-2 gap-2">
                        {formData.options.split(",").map((opt, i) => (
                          <button
                            key={i}
                            className="w-full py-2 bg-[#6b2cc2] hover:bg-[#5a25a8] text-white rounded-lg font-semibold text-sm transition-colors"
                          >
                            {formData.label || "Vote"} {opt.trim()}
                          </button>
                        ))}
                      </div>
                    )}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}