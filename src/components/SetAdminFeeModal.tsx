import { useEffect, useState } from "react";
import Modal from "./Modal";
import { SetAdminFee } from "../interfaces/set-admin-fee";
import { stellarService } from "../services/stellar.service";
import { IRentACarContract } from "../interfaces/contract";
import { useStellarAccounts } from "../providers/StellarAccountProvider";
import { ONE_XLM_IN_STROOPS } from "../utils/xlm-in-stroops";

interface SetAdminFeeModalProps {
  onSetAdminFee: (formData: SetAdminFee) => Promise<void>;
  onCancel: () => void;
}

export const SetAdminFeeModal = ({
  onSetAdminFee,
  onCancel,
}: SetAdminFeeModalProps) => {
  const [isLoading, setIsLoading] = useState(true);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const { walletAddress } = useStellarAccounts();
  const [formData, setFormData] = useState<SetAdminFee>({
    amount: 0,
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type, checked } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]:
        type === "checkbox"
          ? checked
          : type === "number"
            ? Number(value)
            : value,
    }));
  };

  useEffect(() => {
    const fetchAdminFee = async () => {
      const contractClient =
        await stellarService.buildClient<IRentACarContract>(walletAddress);
      const { result } = await contractClient.get_admin_fee();
      setFormData((prev) => ({
        ...prev,
        amount: Number(result) / ONE_XLM_IN_STROOPS,
      }));

      setIsLoading(false);
    };
    void fetchAdminFee();
  }, [walletAddress]);

  const handleSubmit = async (
    e: React.FormEvent<HTMLFormElement>,
  ): Promise<void> => {
    e.preventDefault();
    setIsSubmitting(true);

    try {
      await onSetAdminFee({
        ...formData,
        amount: formData.amount,
      });
    } catch (error) {
      console.error("Error setting admin fee:", error);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <Modal title="Set admin fee" closeModal={onCancel}>
      <div className="bg-white rounded-lg px-8">
        <form onSubmit={(e) => void handleSubmit(e)} className="space-y-4">
          <div>
            <label
              htmlFor="amount"
              className="block text-sm font-medium text-gray-700"
            >
              Amount
            </label>
            <input
              id="amount"
              name="amount"
              type="number"
              min="1"
              value={formData.amount}
              onChange={handleChange}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 p-1"
            />
          </div>

          <div className="flex justify-end gap-4 space-x-3 pt-2 pb-6">
            {onCancel && (
              <button
                type="button"
                onClick={onCancel}
                className="px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 cursor-pointer"
              >
                Cancel
              </button>
            )}
            <button
              type="submit"
              disabled={isSubmitting || isLoading}
              className="px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:bg-gray-400 cursor-pointer"
            >
              {isSubmitting ? "Saving..." : isLoading ? "Loading..." : "Save"}
            </button>
          </div>
        </form>
      </div>
    </Modal>
  );
};
