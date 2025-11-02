import { useState } from "react";
import useModal from "../hooks/useModal";
import { ICar } from "../interfaces/car";
import { CarStatus } from "../interfaces/car-status";
import { IRentACarContract } from "../interfaces/contract";
import { RentCar } from "../interfaces/rent-car";
import { UserRole } from "../interfaces/user-role";
import { useStellarAccounts } from "../providers/StellarAccountProvider";
import { stellarService } from "../services/stellar.service";
import { walletService } from "../services/wallet.service";
import { shortenAddress } from "../utils/shorten-address";
import { RentCarModal } from "./RentCarForm";
import { ONE_XLM_IN_STROOPS } from "../utils/xlm-in-stroops";

interface CarsListProps {
  cars: ICar[];
}

export const CarsList = ({ cars }: CarsListProps) => {
  const { walletAddress, selectedRole, setHashId, setCars } =
    useStellarAccounts();

  const [selectedCard, setSelectedCar] = useState<ICar>();

  const { showModal, openModal, closeModal } = useModal();

  const handleDelete = async (owner: string) => {
    const contractClient =
      await stellarService.buildClient<IRentACarContract>(walletAddress);

    const result = await contractClient.remove_car({ owner });
    const xdr = result.toXDR();

    const signedTx = await walletService.signTransaction(xdr);
    const txHash = await stellarService.submitTransaction(signedTx.signedTxXdr);

    setCars((prev) => prev.filter((car) => car.ownerAddress !== owner));
    setHashId(txHash as string);
  };

  const handlePayout = async (owner: string) => {
    const contractClient =
      await stellarService.buildClient<IRentACarContract>(walletAddress);

    const { result: available_to_withdraw } =
      await contractClient.get_car_available_to_withdraw({
        owner,
      });

    // eslint-disable-next-line @typescript-eslint/unbound-method
    const amount = Number(available_to_withdraw.value);

    const result = await contractClient.payout_owner({ owner, amount });
    const xdr = result.toXDR();

    const signedTx = await walletService.signTransaction(xdr);
    const txHash = await stellarService.submitTransaction(signedTx.signedTxXdr);

    alert(`The ${amount / ONE_XLM_IN_STROOPS} was sent to ${owner}`);

    setHashId(txHash as string);
  };

  const handleRentCar = (car: ICar) => {
    setSelectedCar(car);
    openModal();
  };

  const handleReturn = async (renter: string, owner: string) => {
    const contractClient =
      await stellarService.buildClient<IRentACarContract>(walletAddress);

    const result = await contractClient.return_car({ renter, owner });
    const xdr = result.toXDR();

    const signedTx = await walletService.signTransaction(xdr);
    const txHash = await stellarService.submitTransaction(signedTx.signedTxXdr);

    setCars((prev) =>
      prev.map((c) =>
        c.ownerAddress === owner ? { ...c, status: CarStatus.AVAILABLE } : c,
      ),
    );

    setHashId(txHash as string);
  };

  const handleConfirmRentCar = async (rentCar: RentCar) => {
    const contractClient =
      await stellarService.buildClient<IRentACarContract>(walletAddress);

    const result = await contractClient.rental({
      renter: rentCar.renterAddress,
      owner: rentCar.ownerAddress,
      total_days_to_rent: rentCar.total_days_to_rent,
      amount: rentCar.amount,
    });
    const xdr = result.toXDR();

    const signedTx = await walletService.signTransaction(xdr);
    const txHash = await stellarService.submitTransaction(signedTx.signedTxXdr);

    setCars((prev) =>
      prev.map((c) =>
        c.ownerAddress === rentCar.ownerAddress
          ? { ...c, status: CarStatus.RENTED }
          : c,
      ),
    );

    closeModal();
    setHashId(txHash as string);
  };

  const getStatusStyle = (status: CarStatus): string => {
    switch (status) {
      case CarStatus.AVAILABLE:
        return "px-2 py-1 text-xs font-semibold rounded-full bg-green-100 text-green-800";
      case CarStatus.RENTED:
        return "px-2 py-1 text-xs font-semibold rounded-full bg-blue-100 text-blue-800";
      case CarStatus.MAINTENANCE:
        return "px-2 py-1 text-xs font-semibold rounded-full bg-yellow-100 text-yellow-800";
      default:
        return "px-2 py-1 text-xs font-semibold rounded-full bg-gray-100 text-gray-800";
    }
  };

  const renderActionButton = (car: ICar) => {
    if (selectedRole === UserRole.ADMIN) {
      return (
        <button
          onClick={() => void handleDelete(car.ownerAddress)}
          disabled={car.status != CarStatus.AVAILABLE}
          className="px-3 py-1 bg-red-600 text-white rounded font-semibold hover:bg-red-700 transition-colors cursor-pointer disabled:bg-gray-400 disabled:cursor-not-allowed disabled:hover:bg-gray-400"
        >
          Delete
        </button>
      );
    }

    if (selectedRole === UserRole.OWNER) {
      return (
        <button
          onClick={() => void handlePayout(car.ownerAddress)}
          disabled={car.status !== CarStatus.AVAILABLE}
          className="px-3 py-1 bg-green-600 text-white rounded font-semibold transition-colors cursor-pointer hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed disabled:hover:bg-gray-400"
        >
          Withdraw
        </button>
      );
    }

    if (
      selectedRole === UserRole.RENTER &&
      car.status === CarStatus.AVAILABLE
    ) {
      return (
        <button
          onClick={() => void handleRentCar(car)}
          className="px-3 py-1 bg-green-600 text-white rounded font-semibold hover:bg-green-700 transition-colors cursor-pointer"
        >
          Rent
        </button>
      );
    }

    if (selectedRole === UserRole.RENTER && car.status === CarStatus.RENTED) {
      return (
        <button
          onClick={() => void handleReturn(walletAddress, car.ownerAddress)}
          className="px-3 py-1 bg-blue-600 text-white rounded font-semibold hover:bg-blue-700 transition-colors cursor-pointer"
        >
          Return
        </button>
      );
    }

    return null;
  };

  return (
    <div data-test="cars-list">
      <div>
        <table className="min-w-full bg-white shadow-md rounded-lg">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Brand
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Model
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Color
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Passengers
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                A/C
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Owner
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Price/Day
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Status
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Actions
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {cars.map((car) => (
              <tr
                key={`car-${car.model}-${car.ownerAddress}-row`}
                className="hover:bg-gray-50"
              >
                <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                  {car.brand}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {car.model}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {car.color}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {car.passengers}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {car.ac ? (
                    <span className="text-green-600">Yes</span>
                  ) : (
                    <span className="text-red-600">No</span>
                  )}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {shortenAddress(car.ownerAddress)}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  ${car.pricePerDay}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  <span className={getStatusStyle(car.status)}>
                    {car.status}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {renderActionButton(car)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>

        {showModal && selectedCard && (
          <RentCarModal
            car={selectedCard}
            renter={walletAddress}
            onRentCar={handleConfirmRentCar}
            onCancel={closeModal}
          />
        )}
      </div>
    </div>
  );
};
